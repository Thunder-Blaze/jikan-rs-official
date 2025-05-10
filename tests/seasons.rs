use crate::common::wait_between_tests;
use jikan_rs::JikanClient;
use jikan_rs::{
    seasons::SeasonQueryParams,
    common::enums::season::SeasonFilter,
};
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
    let params = SeasonQueryParams::new()
        .filter(SeasonFilter::TV)
        .sfw(true)
        .page(1)
        .limit(10);
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
    let params = SeasonQueryParams::new()
        .filter(SeasonFilter::Movie)
        .sfw(true)
        .page(1)
        .limit(5);
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
    let params = SeasonQueryParams::new()
        .filter(SeasonFilter::TV)
        .sfw(true)
        .continuing(true)
        .page(1)
        .limit(10);
    let result = client.get_season_upcoming(Some(params)).await;
    assert!(result.is_ok(), "Failed to get upcoming season with filters");
    wait_between_tests().await;
}
