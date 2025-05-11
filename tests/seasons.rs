use crate::common::wait_between_tests;
use jikan_rs::JikanClient;
use jikan_rs::{common::enums::season::SeasonFilter, seasons::SeasonQueryParams};
use serial_test::serial;

mod common;

#[tokio::test]
#[serial]
async fn test_get_season_now() {
    let client = JikanClient::new();
    let result = client.get_season_now(None).await;
    assert!(result.is_ok(), "Failed to get current season");
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn test_get_season_now_with_filters() {
    let client = JikanClient::new();
    let params = SeasonQueryParams {
        filter: Some(SeasonFilter::TV),
        sfw: Some(true),
        page: Some(1),
        limit: Some(10),
        ..Default::default()
    };
    let result = client.get_season_now(Some(params)).await;
    assert!(result.is_ok(), "Failed to get current season with filters");
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn test_get_specific_season() {
    let client = JikanClient::new();
    // Testing with a known season
    let result = client.get_season(2023, "winter", None).await;
    assert!(result.is_ok(), "Failed to get specific season");
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn test_get_specific_season_with_filters() {
    let client = JikanClient::new();
    let params = SeasonQueryParams {
        filter: Some(SeasonFilter::Movie),
        sfw: Some(true),
        page: Some(1),
        limit: Some(5),
        ..Default::default()
    };
    let result = client.get_season(2023, "winter", Some(params)).await;
    assert!(result.is_ok(), "Failed to get specific season with filters");
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn test_get_seasons_list() {
    let client = JikanClient::new();
    let result = client.get_seasons_list().await;
    assert!(result.is_ok(), "Failed to get seasons list");
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn test_get_season_upcoming() {
    let client = JikanClient::new();
    let result = client.get_season_upcoming(None).await;
    assert!(result.is_ok(), "Failed to get upcoming season");
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn test_get_season_upcoming_with_filters() {
    let client = JikanClient::new();
    let params = SeasonQueryParams {
        filter: Some(SeasonFilter::TV),
        sfw: Some(true),
        continuing: Some(true),
        page: Some(1),
        limit: Some(5),
        ..Default::default()
    };
    let result = client.get_season_upcoming(Some(params)).await;
    assert!(result.is_ok(), "Failed to get upcoming season with filters");
    wait_between_tests().await;
}
