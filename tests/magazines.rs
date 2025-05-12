use crate::common::wait_between_tests;
use jikan_rs::{
    JikanClient,
    common::enums::{common::Sort, magazines::MagazineOrder},
};
use serial_test::serial;
mod common;

#[tokio::test]
#[serial]
async fn get_magazines_no_params() {
    let client = JikanClient::new();
    let result = client
        .get_magazines(None, None, None, None, None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_magazines_with_page() {
    let client = JikanClient::new();
    let result = client
        .get_magazines(Some(1), None, None, None, None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_magazines_with_limit() {
    let client = JikanClient::new();
    let result = client
        .get_magazines(None, Some(10), None, None, None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_magazines_with_query() {
    let client = JikanClient::new();
    let result = client
        .get_magazines(None, None, Some("Shonen".to_string()), None, None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_magazines_with_order() {
    let client = JikanClient::new();
    let result = client
        .get_magazines(None, None, None, Some(MagazineOrder::Name), None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_magazines_with_sort() {
    let client = JikanClient::new();
    let result = client
        .get_magazines(None, None, None, Some(MagazineOrder::MalId), Some(Sort::Desc), None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_magazines_with_letter() {
    let client = JikanClient::new();
    let result = client
        .get_magazines(None, None, None, None, None, Some("A".to_string()))
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}
