use idb::{Factory, ObjectStoreParams, TransactionMode};
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
async fn test_transaction_commit() {
    let factory = Factory::new().unwrap();
    factory.delete("test").await.unwrap();

    let mut open_request = factory.open("test", Some(1)).unwrap();
    open_request.on_upgrade_needed(|event| {
        let database = event.database().unwrap();

        database
            .create_object_store("store1", ObjectStoreParams::new())
            .unwrap();
    });

    let database = open_request.await.unwrap();

    let transaction = database
        .transaction(&["store1"], TransactionMode::ReadWrite)
        .unwrap();

    let store = transaction.object_store("store1").unwrap();
    let id = store
        .add(
            &serde_wasm_bindgen::to_value("hello").unwrap(),
            Some(&serde_wasm_bindgen::to_value("world").unwrap()),
        )
        .unwrap()
        .await;

    assert!(id.is_ok(), "id should be ok: {}", id.unwrap_err());
    let id = id.unwrap();
    assert_eq!(serde_wasm_bindgen::to_value("world").unwrap(), id);

    let commit = transaction.commit().await;
    assert!(
        commit.is_ok(),
        "commit should be ok: {}",
        commit.unwrap_err()
    );

    let read_transaction = database
        .transaction(&["store1"], TransactionMode::ReadOnly)
        .unwrap();
    let read_store = read_transaction.object_store("store1").unwrap();

    let count = read_store.count(None).await;
    assert!(count.is_ok(), "count should be ok: {}", count.unwrap_err());
    let count = count.unwrap();

    assert_eq!(count, 1);

    let value = read_store
        .get(serde_wasm_bindgen::to_value("world").unwrap())
        .await;
    assert!(value.is_ok(), "value should be ok: {}", value.unwrap_err());
    let value = value.unwrap().unwrap();

    assert_eq!(value, serde_wasm_bindgen::to_value("hello").unwrap());

    database.close();
    factory.delete("test").await.unwrap();
}

#[wasm_bindgen_test]
async fn test_transaction_abort() {
    let factory = Factory::new().unwrap();
    factory.delete("test").await.unwrap();

    let mut open_request = factory.open("test", Some(1)).unwrap();
    open_request.on_upgrade_needed(|event| {
        let database = event.database().unwrap();

        database
            .create_object_store("store1", ObjectStoreParams::new())
            .unwrap();
    });

    let database = open_request.await.unwrap();

    let transaction = database
        .transaction(&["store1"], TransactionMode::ReadWrite)
        .unwrap();

    let store = transaction.object_store("store1").unwrap();
    let id = store
        .add(
            &serde_wasm_bindgen::to_value("hello").unwrap(),
            Some(&serde_wasm_bindgen::to_value("world").unwrap()),
        )
        .unwrap()
        .await;

    assert!(id.is_ok(), "id should be ok: {}", id.unwrap_err());
    let id = id.unwrap();
    assert_eq!(serde_wasm_bindgen::to_value("world").unwrap(), id);

    let abort = transaction.abort().await;
    assert!(abort.is_ok(), "abort should be ok: {}", abort.unwrap_err());

    let read_transaction = database
        .transaction(&["store1"], TransactionMode::ReadOnly)
        .unwrap();
    let read_store = read_transaction.object_store("store1").unwrap();

    let count = read_store.count(None).await;
    assert!(count.is_ok(), "count should be ok: {}", count.unwrap_err());
    let count = count.unwrap();

    assert_eq!(count, 0);

    database.close();
    factory.delete("test").await.unwrap();
}

#[wasm_bindgen_test]
async fn test_transaction_error() {
    let factory = Factory::new().unwrap();
    factory.delete("test").await.unwrap();

    let mut open_request = factory.open("test", Some(1)).unwrap();
    open_request.on_upgrade_needed(|event| {
        let database = event.database().unwrap();

        database
            .create_object_store("store1", ObjectStoreParams::new())
            .unwrap();
    });

    let mut database = open_request.await.unwrap();
    database.on_version_change(|database| {
        database.close();
    });

    let transaction = database
        .transaction(&["store1"], TransactionMode::ReadWrite)
        .unwrap();

    let store = transaction.object_store("store1").unwrap();
    store
        .add(
            &serde_wasm_bindgen::to_value("hello").unwrap(),
            Some(&serde_wasm_bindgen::to_value("world").unwrap()),
        )
        .unwrap()
        .await
        .unwrap();

    let mut open_request = factory.open("test", Some(2)).unwrap();
    open_request.on_upgrade_needed(|event| {
        let database = event.database().unwrap();

        database
            .create_object_store("store2", ObjectStoreParams::new())
            .unwrap();
    });

    let database = open_request.await.unwrap();

    let error = transaction.commit().await;
    assert!(error.is_err(), "commit should fail"); // Note that the value may still get committed to the store depending on indexedDB implementation on the browser

    database.close();
    factory.delete("test").await.unwrap();
}

#[wasm_bindgen_test]
async fn test_transaction_read_write_in_loop() {
    let factory = Factory::new().unwrap();
    factory.delete("test").await.unwrap();

    let mut open_request = factory.open("test", Some(1)).unwrap();
    open_request.on_upgrade_needed(|event| {
        let database = event.database().unwrap();

        database
            .create_object_store("store1", ObjectStoreParams::new())
            .unwrap();
    });

    let database = open_request.await.unwrap();

    // 1. Insert 10 values in database

    let transaction = database
        .transaction(&["store1"], TransactionMode::ReadWrite)
        .unwrap();

    let store = transaction.object_store("store1").unwrap();

    for i in 0..10 {
        let id = store
            .add(
                &serde_wasm_bindgen::to_value("hello").unwrap(),
                Some(&serde_wasm_bindgen::to_value(&i.to_string()).unwrap()),
            )
            .unwrap()
            .await;

        assert!(id.is_ok(), "id should be ok: {}", id.unwrap_err());
        let id = id.unwrap();
        assert_eq!(serde_wasm_bindgen::to_value(&i.to_string()).unwrap(), id);
    }

    let commit = transaction.commit().await;
    assert!(
        commit.is_ok(),
        "commit should be ok: {}",
        commit.unwrap_err()
    );

    // 2. Change all the values in database

    let transaction = database
        .transaction(&["store1"], TransactionMode::ReadWrite)
        .unwrap();

    let store = transaction.object_store("store1").unwrap();

    for i in 0..10 {
        let value = store
            .get(serde_wasm_bindgen::to_value(&i.to_string()).unwrap())
            .await;

        assert!(value.is_ok(), "value should be ok: {}", value.unwrap_err());
        let value = value.unwrap();

        assert!(value.is_some(), "value should be some");
        let value = value.unwrap();
        assert_eq!(value, serde_wasm_bindgen::to_value("hello").unwrap());

        let id = store
            .put(
                &serde_wasm_bindgen::to_value("hello").unwrap(),
                Some(&serde_wasm_bindgen::to_value(&i.to_string()).unwrap()),
            )
            .unwrap()
            .await;

        assert!(id.is_ok(), "id should be ok: {}", id.unwrap_err());
        let id = id.unwrap();
        assert_eq!(serde_wasm_bindgen::to_value(&i.to_string()).unwrap(), id);
    }

    transaction.commit().await.unwrap();

    database.close();
    factory.delete("test").await.unwrap();
}
