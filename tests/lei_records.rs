use gleif_rs::{
    client::GleifClient,
    field::Field,
    model::{LeiRecord, LeiRecordList},
    value::{EntityCategory, RegistrationStatus},
};

#[tokio::test]
async fn test_lei_record_by_id() {
    let client = GleifClient::new();
    let lei = "5493001KJTIIGC8Y1R12"; // Example LEI from Bloomberg Finance L.P.
    let result = client.lei_record_by_id(lei).await;
    assert!(
        result.is_ok(),
        "Expected Ok result from lei_record_by_id, got: {:?}",
        result
    );
    let value: serde_json::Value = result.unwrap();
    // Check that the response contains a 'data' field
    let data = value.get("data");
    assert!(data.is_some(), "Missing 'data' field in response");

    let result_typed = client.lei_record_by_id::<LeiRecord>(lei).await;
    assert!(
        result_typed.is_ok(),
        "Expected Ok result from strongly typed lei_record_by_id, got: {:?}",
        result_typed
    );
    let record = result_typed.unwrap();
    // Check that the LEI in the strongly typed response matches the requested LEI
    assert_eq!(
        record.data.attributes.lei, lei,
        "LEI in strongly typed response does not match requested LEI"
    );
}

#[tokio::test]
async fn test_lei_records_endpoint() {
    let client = GleifClient::new();
    let result = client
        .lei_records()
        .filter_eq(Field::EntityCategory, EntityCategory::Fund)
        .filter_eq(Field::RegistrationStatus, RegistrationStatus::Issued)
        .sort(Field::EntityLegalName)
        .page_size(3)
        .send()
        .await;
    assert!(
        result.is_ok(),
        "Expected Ok result from lei_records, got: {:?}",
        result
    );
    let value: serde_json::Value = result.unwrap();
    // Check that the response contains a 'data' field which is an array
    assert!(
        value.get("data").is_some(),
        "Expected 'data' field in response, got: {:?}",
        value
    );
    let data = value.get("data").expect("Missing 'data' field in response");
    assert!(data.is_array(), "'data' field is not an array");

    // Strongly typed example
    let result_typed = client
        .lei_records()
        .filter_eq(Field::EntityCategory, EntityCategory::Fund)
        .filter_eq(Field::RegistrationStatus, RegistrationStatus::Issued)
        .sort(Field::EntityLegalName)
        .page_size(3)
        .send::<LeiRecordList>()
        .await;

    assert!(
        result_typed.is_ok(),
        "Expected Ok result from strongly typed lei_records endpoint, got: {:?}",
        result_typed
    );
    let list = result_typed.unwrap();
    // Check that the number of records is at most 3
    assert!(
        list.data.len() <= 3,
        "More than 3 records returned in strongly typed response"
    );
}
