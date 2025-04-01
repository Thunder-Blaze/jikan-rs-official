use crate::common::wait_between_tests;
use jikan_rs::JikanClient;
use serial_test::serial;

mod common;

#[tokio::test]
#[serial]
async fn test_get_watch_recent_episodes() {
    let client = JikanClient::new();
    let result = client.get_watch_recent_episodes().await;

    match result {
        Ok(response) => {
            assert!(
                !response.data.is_empty(),
                "Recent episodes response should not be empty"
            );
            assert!(
                response.pagination.last_visible_page >= 1,
                "Should have a valid page number"
            );
        }
        Err(e) => panic!("Failed to fetch recent episodes: {}", e),
    }

    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn test_get_watch_popular_episodes() {
    let client = JikanClient::new();
    let result = client.get_watch_popular_episodes().await;

    match result {
        Ok(response) => {
            assert!(
                !response.data.is_empty(),
                "Popular episodes response should not be empty"
            );
            assert!(
                response.pagination.last_visible_page >= 1,
                "Should have a valid page number"
            );
        }
        Err(e) => panic!("Failed to fetch popular episodes: {}", e),
    }

    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn test_get_watch_recent_promos_no_page() {
    let client = JikanClient::new();
    let result = client.get_watch_recent_promos(None).await;

    match result {
        Ok(response) => {
            assert!(
                !response.data.is_empty(),
                "Recent promos response should not be empty"
            );
            assert!(
                response.pagination.last_visible_page >= 1,
                "Should have a valid page number"
            );
            assert!(!response.title.is_empty(), "Response should have a title");
        }
        Err(e) => panic!("Failed to fetch recent promos without page: {}", e),
    }

    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn test_get_watch_recent_promos_with_page() {
    let client = JikanClient::new();
    let result = client.get_watch_recent_promos(Some(1)).await;

    match result {
        Ok(response) => {
            assert!(
                !response.data.is_empty(),
                "Recent promos response should not be empty"
            );
            assert!(
                response.pagination.last_visible_page >= 1,
                "Should have a valid page number"
            );
            assert!(!response.title.is_empty(), "Response should have a title");
        }
        Err(e) => panic!("Failed to fetch recent promos with page: {}", e),
    }

    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn test_get_watch_popular_promos() {
    let client = JikanClient::new();
    let result = client.get_watch_popular_promos().await;

    match result {
        Ok(response) => {
            assert!(
                !response.data.is_empty(),
                "Popular promos response should not be empty"
            );
            assert!(
                response.pagination.last_visible_page >= 1,
                "Should have a valid page number"
            );
            assert!(!response.title.is_empty(), "Response should have a title");
        }
        Err(e) => panic!("Failed to fetch popular promos: {}", e),
    }

    wait_between_tests().await;
}
