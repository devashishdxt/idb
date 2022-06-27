mod database;
mod factory;
mod open_request;

use idb::{
    Database, Error, Factory, IndexParams, KeyPath, KeyRange, ObjectStoreParams, TransactionMode,
};
use js_sys::Array;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;
use wasm_bindgen_test::{wasm_bindgen_test, wasm_bindgen_test_configure};

wasm_bindgen_test_configure!(run_in_browser);

#[derive(Debug, Deserialize, PartialEq)]
struct Employee {
    id: u32,
    name: String,
    email: String,
}

#[derive(Debug, Serialize)]
struct EmployeeRequest<'a> {
    name: &'a str,
    email: &'a str,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Invoice {
    id: usize,
    year: u16,
    agent: String,
    customer: String,
}

#[derive(Debug, Serialize)]
struct InvoiceRequest<'a> {
    id: usize,
    year: u16,
    agent: &'a str,
    customer: &'a str,
}

/// Creates a database
async fn create_db() -> Result<Database, Error> {
    let factory = Factory::new()?;
    factory.delete("test").await?;

    let open_request = factory.open("test", 1);
    assert!(open_request.is_ok());
    let mut open_request = open_request.unwrap();

    open_request.on_upgrade_needed(|event| {
        // Get database handle
        let database = event.database();
        assert!(database.is_ok());
        let database = database.unwrap();

        // Create employees object store
        let mut employees_params = ObjectStoreParams::new();
        employees_params
            .key_path(Some(KeyPath::new_single("id")))
            .auto_increment(true);
        let employees = database.create_object_store("employees", &employees_params);
        assert!(employees.is_ok());
        let employees = employees.unwrap();

        // Add email index to employees object store
        let mut email_index_params = IndexParams::new();
        email_index_params.unique(true);
        let email_index = employees.create_index(
            "email",
            KeyPath::new_single("email"),
            Some(email_index_params),
        );
        assert!(email_index.is_ok());

        // Create departments object store
        let mut departments_params = ObjectStoreParams::new();
        departments_params.auto_increment(true);
        let departments = database.create_object_store("departments", &departments_params);
        assert!(departments.is_ok());

        // Create invoices object store
        let mut invoices_params = ObjectStoreParams::new();
        invoices_params.key_path(Some(KeyPath::new_array(["id", "year"])));
        let invoices = database.create_object_store("invoices", &invoices_params);
        assert!(invoices.is_ok());
        let invoices = invoices.unwrap();

        // Add agent_customer index to invoices object store
        let agent_customer_index = invoices.create_index(
            "agent_customer",
            KeyPath::new_array(["agent", "customer"]),
            None,
        );
        assert!(agent_customer_index.is_ok());
    });

    open_request.execute().await
}

pub async fn cleanup(database: Database) -> Result<(), Error> {
    database.close();

    let factory = Factory::new()?;
    factory.delete("test").await
}

async fn add_employee(database: &Database, name: &str, email: &str) -> Result<u32, Error> {
    let transaction = database.transaction(&["employees"], TransactionMode::ReadWrite);
    assert!(transaction.is_ok());
    let transaction = transaction.unwrap();

    let employees = transaction.object_store("employees");
    assert!(employees.is_ok());
    let employees = employees.unwrap();

    let employee = EmployeeRequest { name, email };
    let employee = serde_wasm_bindgen::to_value(&employee).unwrap();
    let employee_id = employees.add(&employee, None).await?;

    transaction.commit().await?;
    Ok(num_traits::cast(employee_id.as_f64().unwrap()).unwrap())
}

async fn get_employee(database: &Database, id: u32) -> Result<Option<Employee>, Error> {
    let transaction = database.transaction(&["employees"], TransactionMode::ReadOnly);
    assert!(transaction.is_ok());
    let transaction = transaction.unwrap();

    let employees = transaction.object_store("employees");
    assert!(employees.is_ok());
    let employees = employees.unwrap();

    let employee = employees
        .get(JsValue::from_f64(num_traits::cast(id).unwrap()))
        .await?;
    let employee: Option<Employee> = serde_wasm_bindgen::from_value(employee).unwrap();

    Ok(employee)
}

async fn get_all_employees(database: &Database) -> Result<Vec<Employee>, Error> {
    let transaction = database.transaction(&["employees"], TransactionMode::ReadOnly);
    assert!(transaction.is_ok());
    let transaction = transaction.unwrap();

    let employees = transaction.object_store("employees");
    assert!(employees.is_ok());
    let employees = employees.unwrap();

    let employees: Vec<JsValue> = employees.get_all(None, None).await?;

    let employees: Vec<Employee> = employees
        .into_iter()
        .map(|employee| serde_wasm_bindgen::from_value(employee).unwrap())
        .collect();

    Ok(employees)
}

async fn count_employees(database: &Database, key_range: Option<KeyRange>) -> Result<u32, Error> {
    let transaction = database.transaction(&["employees"], TransactionMode::ReadOnly);
    assert!(transaction.is_ok());
    let transaction = transaction.unwrap();

    let employees = transaction.object_store("employees");
    assert!(employees.is_ok());
    let employees = employees.unwrap();

    employees.count(key_range.map(Into::into)).await
}

async fn clear_employees(database: &Database) -> Result<(), Error> {
    let transaction = database.transaction(&["employees"], TransactionMode::ReadWrite);
    assert!(transaction.is_ok());
    let transaction = transaction.unwrap();

    let employees = transaction.object_store("employees");
    assert!(employees.is_ok());
    let employees = employees.unwrap();

    employees.clear().await
}

async fn add_invoice(
    database: &Database,
    id: usize,
    year: u16,
    agent: &str,
    customer: &str,
) -> Result<(), Error> {
    let transaction = database.transaction(&["invoices"], TransactionMode::ReadWrite);
    assert!(transaction.is_ok());
    let transaction = transaction.unwrap();

    let invoices = transaction.object_store("invoices");
    assert!(invoices.is_ok());
    let invoices = invoices.unwrap();

    let invoice = InvoiceRequest {
        id,
        year,
        agent,
        customer,
    };
    let invoice = serde_wasm_bindgen::to_value(&invoice).unwrap();
    invoices.add(&invoice, None).await?;

    transaction.commit().await?;
    Ok(())
}

async fn get_invoice(database: &Database, id: usize, year: u16) -> Result<Option<Invoice>, Error> {
    let transaction = database.transaction(&["invoices"], TransactionMode::ReadOnly);
    assert!(transaction.is_ok());
    let transaction = transaction.unwrap();

    let invoices = transaction.object_store("invoices");
    assert!(invoices.is_ok());
    let invoices = invoices.unwrap();

    let invoice = invoices
        .get(JsValue::from(Array::of2(
            &JsValue::from_f64(id as _),
            &JsValue::from_f64(year as _),
        )))
        .await?;
    let invoice: Option<Invoice> = serde_wasm_bindgen::from_value(invoice).unwrap();

    Ok(invoice)
}

async fn get_all_invoices_by_agent_and_customer(
    database: &Database,
    agent: &str,
    customer: &str,
) -> Result<Vec<Invoice>, Error> {
    let transaction = database.transaction(&["invoices"], TransactionMode::ReadOnly);
    assert!(transaction.is_ok());
    let transaction = transaction.unwrap();

    let invoices = transaction.object_store("invoices");
    assert!(invoices.is_ok());
    let invoices = invoices.unwrap();

    let agent_customer_index = invoices.index("agent_customer");
    assert!(agent_customer_index.is_ok());
    let agent_customer_index = agent_customer_index.unwrap();

    let invoices = agent_customer_index
        .get_all(
            Some(
                KeyRange::only(&Array::of2(&agent.into(), &customer.into()))
                    .unwrap()
                    .into(),
            ),
            None,
        )
        .await?;
    let invoices = invoices
        .into_iter()
        .map(|value| serde_wasm_bindgen::from_value(value).unwrap())
        .collect();

    Ok(invoices)
}

#[wasm_bindgen_test]
async fn test_create_db_pass() {
    let database = create_db().await;
    assert!(database.is_ok());
    let database = database.unwrap();

    assert_eq!(database.name(), "test");
    assert_eq!(database.version().expect("version"), 1);

    let store_names = database.store_names();
    assert_eq!(store_names.len(), 3);

    cleanup(database).await.expect("cleanup");
}

#[wasm_bindgen_test]
async fn test_db_add_pass() {
    let database = create_db().await;
    assert!(database.is_ok());
    let database = database.unwrap();

    // Write values to the database.
    let id = add_employee(&database, "John Doe", "john@example.com").await;
    assert_eq!(id, Ok(1));

    let id2 = add_employee(&database, "Scooby Doo", "scooby@example.com").await;
    assert_eq!(id2, Ok(2));

    // Read the values back from the database.
    let employee = get_employee(&database, 1).await;
    assert!(employee.is_ok());
    let employee = employee.unwrap();
    assert!(employee.is_some());
    let employee = employee.unwrap();

    assert_eq!(employee.id, 1);
    assert_eq!(employee.name, "John Doe");
    assert_eq!(employee.email, "john@example.com");

    let employee = get_employee(&database, 2).await;
    assert!(employee.is_ok());
    let employee = employee.unwrap();
    assert!(employee.is_some());
    let employee = employee.unwrap();

    assert_eq!(employee.id, 2);
    assert_eq!(employee.name, "Scooby Doo");
    assert_eq!(employee.email, "scooby@example.com");

    let employee = get_employee(&database, 3).await;
    assert!(employee.is_ok());
    let employee = employee.unwrap();
    assert!(employee.is_none());

    let ok = add_invoice(&database, 1, 2022, "John Doe", "Umbrella Corp").await;
    assert!(ok.is_ok());
    let ok = add_invoice(&database, 1, 2023, "Scooby Doo", "Umbrella Corp").await;
    assert!(ok.is_ok());

    let invoice = get_invoice(&database, 1, 2022).await;
    assert!(invoice.is_ok());
    let invoice = invoice.unwrap();
    assert!(invoice.is_some());
    let invoice = invoice.unwrap();

    assert_eq!(invoice.id, 1);
    assert_eq!(invoice.year, 2022);
    assert_eq!(invoice.agent, "John Doe");
    assert_eq!(invoice.customer, "Umbrella Corp");

    cleanup(database).await.expect("cleanup");
}

#[wasm_bindgen_test]
async fn test_db_duplicate_add_fail() {
    let database = create_db().await;
    assert!(database.is_ok());
    let database = database.unwrap();

    // Write a value to the database.
    let id = add_employee(&database, "John Doe", "john@example.com").await;
    assert_eq!(id, Ok(1));

    // Write a duplicate value (with same email) to the database.
    let id = add_employee(&database, "John Doe New", "john@example.com").await;
    assert!(id.is_err());
    let err = id.unwrap_err();
    assert!(err
        .to_string()
        .starts_with("DOM exception: ConstraintError"));

    cleanup(database).await.expect("cleanup");
}

#[wasm_bindgen_test]
async fn test_db_count_and_clear_pass() {
    let database = create_db().await;
    assert!(database.is_ok());
    let database = database.unwrap();

    // Write values to the database.
    let id = add_employee(&database, "John Doe", "john@example.com").await;
    assert_eq!(id, Ok(1));

    let id2 = add_employee(&database, "Scooby Doo", "scooby@example.com").await;
    assert_eq!(id2, Ok(2));

    // Count the number of values in the database before and after clearing.
    assert_eq!(count_employees(&database, None).await, Ok(2));
    assert_eq!(
        count_employees(&database, Some(KeyRange::only(&1u32.into()).unwrap())).await,
        Ok(1)
    );
    assert_eq!(
        count_employees(
            &database,
            Some(KeyRange::lower_bound(&1u32.into(), Some(true)).unwrap())
        )
        .await,
        Ok(1)
    );
    assert_eq!(
        count_employees(
            &database,
            Some(KeyRange::lower_bound(&2u32.into(), Some(false)).unwrap())
        )
        .await,
        Ok(1)
    );
    assert_eq!(
        count_employees(
            &database,
            Some(KeyRange::lower_bound(&2u32.into(), Some(true)).unwrap())
        )
        .await,
        Ok(0)
    );
    assert!(clear_employees(&database).await.is_ok());
    assert_eq!(count_employees(&database, None).await, Ok(0));

    cleanup(database).await.expect("cleanup");
}

#[wasm_bindgen_test]
async fn test_get_all_pass() {
    let database = create_db().await;
    assert!(database.is_ok());
    let database = database.unwrap();

    // Write values to the database.
    let id = add_employee(&database, "John Doe", "john@example.com").await;
    assert_eq!(id, Ok(1));

    let id2 = add_employee(&database, "Scooby Doo", "scooby@example.com").await;
    assert_eq!(id2, Ok(2));

    let employees = get_all_employees(&database).await;
    assert!(employees.is_ok());
    let employees = employees.unwrap();
    assert_eq!(employees.len(), 2);

    // TODO: check employee details

    let ok = add_invoice(&database, 1, 2022, "John Doe", "Umbrella Corp").await;
    assert!(ok.is_ok());
    let ok = add_invoice(&database, 2, 2022, "Scooby Doo", "Umbrella Corp").await;
    assert!(ok.is_ok());
    let ok = add_invoice(&database, 3, 2022, "John Doe", "Umbrella Corp").await;
    assert!(ok.is_ok());

    let invoices =
        get_all_invoices_by_agent_and_customer(&database, "John Doe", "Umbrella Corp").await;
    assert!(invoices.is_ok());
    let invoices = invoices.unwrap();
    assert_eq!(invoices.len(), 2);
    for invoice in invoices {
        assert!(invoice.id == 1 || invoice.id == 3);
        assert_eq!(invoice.year, 2022);
    }

    cleanup(database).await.expect("cleanup");
}

#[wasm_bindgen_test]
async fn check_transaction_abort() {
    let database = create_db().await;
    assert!(database.is_ok());
    let database = database.unwrap();

    let transaction = database.transaction(&["employees"], TransactionMode::ReadWrite);
    assert!(transaction.is_ok());
    let transaction = transaction.unwrap();

    let employees = transaction.object_store("employees");
    assert!(employees.is_ok());
    let employees = employees.unwrap();

    let employee = EmployeeRequest {
        name: "John Doe",
        email: "john@example.com",
    };
    let employee = serde_wasm_bindgen::to_value(&employee).unwrap();
    assert!(employees.add(&employee, None).await.is_ok());

    assert!(transaction.abort().await.is_ok());

    let employees = get_all_employees(&database).await;
    assert!(employees.is_ok());
    let employees = employees.unwrap();

    assert!(employees.is_empty());

    let id = add_employee(&database, "Scooby Doo", "scooby@example.com").await;
    assert_eq!(id, Ok(1));

    let employees = get_all_employees(&database).await;
    assert!(employees.is_ok());
    let employees = employees.unwrap();

    assert_eq!(employees.len(), 1);

    cleanup(database).await.expect("cleanup");
}
