use crate::common::wait_between_tests;
use jikan_rs::JikanClient;
use serial_test::serial;
mod common;

#[tokio::test]
#[serial]
async fn get_person_from_id() {
    let client = JikanClient::new();
    let result = client.get_person_by_id(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_person_full() {
    let client = JikanClient::new();
    let result = client.get_person_by_id_full(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_person_anime() {
    let client = JikanClient::new();
    let result = client.get_person_anime(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_person_manga() {
    let client = JikanClient::new();
    let result = client.get_person_manga(2619).await; // Araki, Hirohiko
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_person_voice() {
    let client = JikanClient::new();
    let result = client.get_person_voice(195).await;
    assert!(result.is_ok()); // Junichi Suwabe(Ryomen Sukuna)
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_person_pictures() {
    let client = JikanClient::new();
    let result = client.get_person_pictures(1).await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_person() {
    let client = JikanClient::new();
    let result = client.get_people().await;
    assert!(result.is_ok());
    wait_between_tests().await;
}

#[tokio::test]
#[serial]
async fn get_person_search() {
    let client = JikanClient::new();
    let result = client
        .get_people_search(
            None,
            None,
            Some(String::from("Junichi Suwabe")),
            None,
            None,
            None,
        )
        .await;
    assert!(result.is_ok());
    wait_between_tests().await;
}
