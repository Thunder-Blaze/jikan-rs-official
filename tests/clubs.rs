use crate::common::wait_between_tests;
use jikan_rs::{
    JikanClient, JikanError,
    clubs::ClubSearchParams,
    common::enums::{
        clubs::{ClubOrder, ClubType},
        common::Sort,
    },
};
use serial_test::serial;
mod common;

#[tokio::test]
#[serial]
async fn get_club_by_id() {
    let client = JikanClient::new();
    let result = client.get_club_by_id(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_club_members() {
    let client = JikanClient::new();
    let result = client.get_club_members(1, Some(1)).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_club_staff() {
    let client = JikanClient::new();
    let result = client.get_club_staff(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_club_relations() {
    let client = JikanClient::new();
    let result = client.get_club_relations(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_club_search() {
    let client = JikanClient::new();
    let params = ClubSearchParams {
        q: Some("test".to_string()),
        page: Some(1),
        limit: Some(10),
        ..Default::default()
    };
    let result = client.get_club_search(Some(params)).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_club_search_with_category() {
    let client = JikanClient::new();
    let params = ClubSearchParams {
        page: Some(1),
        limit: Some(10),
        ..Default::default()
    };
    let result = client.get_club_search(Some(params)).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_club_search_with_order() {
    let client = JikanClient::new();
    let params = ClubSearchParams {
        page: Some(1),
        limit: Some(10),
        order_by: Some(ClubOrder::MembersCount),
        sort: Some(Sort::Desc),
        ..Default::default()
    };
    let result = client.get_club_search(Some(params)).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_club_search_with_letter() {
    let client = JikanClient::new();
    let params = ClubSearchParams {
        page: Some(1),
        limit: Some(10),
        type_: Some(ClubType::Public),
        letter: Some("A".to_string()),
        ..Default::default()
    };
    let result = client.get_club_search(Some(params)).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_nonexistent_club() {
    let client = JikanClient::new();
    let result = client.get_club_by_id(999999999).await;
    assert!(matches!(result, Err(JikanError::NotFound)));
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_nonexistent_club_members() {
    let client = JikanClient::new();
    let result = client.get_club_members(999999999, Some(1)).await;
    assert!(matches!(result, Err(JikanError::NotFound)));
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_nonexistent_club_staff() {
    let client = JikanClient::new();
    let result = client.get_club_staff(999999999).await;
    assert!(matches!(result, Err(JikanError::NotFound)));
    wait_between_tests().await;
}
