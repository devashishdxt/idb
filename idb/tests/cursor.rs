use idb::{CursorDirection, Factory, IndexParams, KeyPath, ObjectStoreParams, TransactionMode};
use serde::Serialize;
use serde_wasm_bindgen::Serializer;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
async fn test_cursor_next_advance_and_get() {
    let factory = Factory::new().unwrap();
    factory.delete("test").await.unwrap();

    let mut open_request = factory.open("test", Some(1)).unwrap();
    open_request.on_upgrade_needed(|event| {
        let database = event.database().unwrap();

        let mut store_params = ObjectStoreParams::new();
        store_params.auto_increment(true);
        store_params.key_path(Some(KeyPath::new_single("id")));

        let store = database
            .create_object_store("employees", store_params)
            .unwrap();

        let mut index_params = IndexParams::new();
        index_params.unique(true);

        store
            .create_index("email", KeyPath::new_single("email"), Some(index_params))
            .unwrap();
    });

    let database = open_request.await.unwrap();

    // Insert multiple values
    let transaction = database
        .transaction(&["employees"], TransactionMode::ReadWrite)
        .unwrap();

    let store = transaction.object_store("employees").unwrap();

    let employee1 = serde_json::json!({
        "name": "John Doe",
        "email": "john@example.com",
    });
    let employee2 = serde_json::json!({
        "name": "Jane Doe",
        "email": "jane@example.com",
    });

    let id1 = store
        .add(
            &employee1.serialize(&Serializer::json_compatible()).unwrap(),
            None,
        )
        .await
        .unwrap();
    let id2 = store
        .add(
            &employee2.serialize(&Serializer::json_compatible()).unwrap(),
            None,
        )
        .await
        .unwrap();

    transaction.commit().await.unwrap();

    // Test cursor flow (with next)
    let transaction = database
        .transaction(&["employees"], TransactionMode::ReadOnly)
        .unwrap();

    let store = transaction.object_store("employees").unwrap();

    let mut cursor = store
        .open_cursor(None, Some(CursorDirection::Next))
        .await
        .unwrap()
        .unwrap();

    assert_eq!(Ok(id1.clone()), cursor.key());
    cursor.next(None).await.unwrap();
    assert_eq!(Ok(id2.clone()), cursor.key());
    cursor.next(None).await.unwrap();
    assert_eq!(Ok(JsValue::null()), cursor.key());

    assert!(cursor.next(None).await.is_err());

    transaction.done().await.unwrap();

    // Test cursor flow (with advance)
    let transaction = database
        .transaction(&["employees"], TransactionMode::ReadOnly)
        .unwrap();

    let store = transaction.object_store("employees").unwrap();

    let mut cursor = store
        .open_cursor(None, Some(CursorDirection::Next))
        .await
        .unwrap()
        .unwrap();

    assert_eq!(Ok(id1), cursor.key());
    cursor.advance(1).await.unwrap();
    assert_eq!(Ok(id2), cursor.key());
    cursor.advance(1).await.unwrap();
    assert_eq!(Ok(JsValue::null()), cursor.key());

    assert!(cursor.advance(1).await.is_err());

    transaction.done().await.unwrap();

    database.close();
    factory.delete("test").await.unwrap();
}

#[wasm_bindgen_test]
async fn test_cursor_delete() {
    let factory = Factory::new().unwrap();
    factory.delete("test").await.unwrap();

    let mut open_request = factory.open("test", Some(1)).unwrap();
    open_request.on_upgrade_needed(|event| {
        let database = event.database().unwrap();

        let mut store_params = ObjectStoreParams::new();
        store_params.auto_increment(true);
        store_params.key_path(Some(KeyPath::new_single("id")));

        let store = database
            .create_object_store("employees", store_params)
            .unwrap();

        let mut index_params = IndexParams::new();
        index_params.unique(true);

        store
            .create_index("email", KeyPath::new_single("email"), Some(index_params))
            .unwrap();
    });

    let database = open_request.await.unwrap();

    // Insert multiple values
    let transaction = database
        .transaction(&["employees"], TransactionMode::ReadWrite)
        .unwrap();

    let store = transaction.object_store("employees").unwrap();

    let employee1 = serde_json::json!({
        "name": "John Doe",
        "email": "john@example.com",
    });
    let employee2 = serde_json::json!({
        "name": "Jane Doe",
        "email": "jane@example.com",
    });

    let id1 = store
        .add(
            &employee1.serialize(&Serializer::json_compatible()).unwrap(),
            None,
        )
        .await
        .unwrap();
    let id2 = store
        .add(
            &employee2.serialize(&Serializer::json_compatible()).unwrap(),
            None,
        )
        .await
        .unwrap();

    transaction.commit().await.unwrap();

    // Delete the second key using cursor
    let transaction = database
        .transaction(&["employees"], TransactionMode::ReadWrite)
        .unwrap();

    let store = transaction.object_store("employees").unwrap();

    let mut cursor = store
        .open_cursor(None, Some(CursorDirection::Next))
        .await
        .unwrap()
        .unwrap();

    assert_eq!(Ok(id1.clone()), cursor.key());
    cursor.next(None).await.unwrap();
    assert_eq!(Ok(id2.clone()), cursor.key());

    cursor.delete().await.unwrap();

    cursor.next(None).await.unwrap();
    assert_eq!(Ok(JsValue::null()), cursor.key());

    assert!(cursor.next(None).await.is_err());

    transaction.commit().await.unwrap();

    // Get count of values in store
    let transaction = database
        .transaction(&["employees"], TransactionMode::ReadOnly)
        .unwrap();

    let store = transaction.object_store("employees").unwrap();

    let count = store.count(None).await.unwrap();
    assert_eq!(1, count);

    database.close();
    factory.delete("test").await.unwrap();
}

#[wasm_bindgen_test]
async fn test_cursor_with_zero_matches() {
    let factory = Factory::new().unwrap();
    factory.delete("test").await.unwrap();

    let mut open_request = factory.open("test", Some(1)).unwrap();
    open_request.on_upgrade_needed(|event| {
        let database = event.database().unwrap();

        let mut store_params = ObjectStoreParams::new();
        store_params.auto_increment(true);
        store_params.key_path(Some(KeyPath::new_single("id")));

        let store = database
            .create_object_store("employees", store_params)
            .unwrap();

        let mut index_params = IndexParams::new();
        index_params.unique(true);

        store
            .create_index("email", KeyPath::new_single("email"), Some(index_params))
            .unwrap();
    });

    let database = open_request.await.unwrap();

    let transaction = database
        .transaction(&["employees"], TransactionMode::ReadOnly)
        .unwrap();

    let store = transaction.object_store("employees").unwrap();

    assert!(store.open_cursor(None, None).await.unwrap().is_none());
}
