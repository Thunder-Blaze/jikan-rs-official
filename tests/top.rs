use crate::common::wait_between_tests;
use jikan_rs::{JikanClient, common::enums::{
    anime::{AnimeType, AnimeFilter, AnimeRating},
    manga::{MangaType, MangaFilter},
    reviews::ReviewType,
}};
use serial_test::serial;
mod common;

#[tokio::test]
#[serial]
async fn get_top_anime() {
    let client = JikanClient::new();
    let result = client
        .get_top_anime(
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_anime_with_type() {
    let client = JikanClient::new();
    let result = client
        .get_top_anime(Some(AnimeType::TV), None, None, None, None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_anime_with_filter() {
    let client = JikanClient::new();
    let result = client
        .get_top_anime(
            None,
            Some(AnimeFilter::Airing),
            None,
            None,
            None,
            None,
        )
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_anime_with_rating() {
    let client = JikanClient::new();
    let result = client
        .get_top_anime(
            None,
            None,
            Some(AnimeRating::PG13),
            None,
            None,
            None,
        )
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_anime_with_sfw() {
    let client = JikanClient::new();
    let result = client
        .get_top_anime(
            None,
            None,
            None,
            Some(true),
            None,
            None,
        )
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_anime_with_page() {
    let client = JikanClient::new();
    let result = client
        .get_top_anime(
            None,
            None,
            None,
            None,
            Some(1),
            None,
        )
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_anime_with_limit() {
    let client = JikanClient::new();
    let result = client
        .get_top_anime(
            None,
            None,
            None,
            None,
            None,
            Some(10),
        )
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_manga() {
    let client = JikanClient::new();
    let result = client
        .get_top_manga(None, None, None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_manga_with_type() {
    let client = JikanClient::new();
    let result = client
        .get_top_manga(Some(MangaType::Manga), None, None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_manga_with_filter() {
    let client = JikanClient::new();
    let result = client
        .get_top_manga(None, Some(MangaFilter::Publishing), None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_manga_with_page() {
    let client = JikanClient::new();
    let result = client
        .get_top_manga(None, None, Some(1), None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_manga_with_limit() {
    let client = JikanClient::new();
    let result = client
        .get_top_manga(None, None, None, Some(10))
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_people() {
    let client = JikanClient::new();
    let result = client.get_top_people(None, None).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_people_with_page() {
    let client = JikanClient::new();
    let result = client.get_top_people(Some(1), None).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_people_with_limit() {
    let client = JikanClient::new();
    let result = client.get_top_people(None, Some(10)).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_reviews() {
    let client = JikanClient::new();
    let result = client
        .get_top_reviews(None, None, None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_reviews_with_type() {
    let client = JikanClient::new();
    let result = client
        .get_top_reviews(Some(ReviewType::Anime), None, None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_reviews_with_preliminary() {
    let client = JikanClient::new();
    let result = client
        .get_top_reviews(None, Some(true), None, None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_reviews_with_spoilers() {
    let client = JikanClient::new();
    let result = client
        .get_top_reviews(None, None, Some(true), None)
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_top_reviews_with_page() {
    let client = JikanClient::new();
    let result = client
        .get_top_reviews(None, None, None, Some(2))
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}
