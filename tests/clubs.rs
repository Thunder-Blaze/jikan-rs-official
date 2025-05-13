use jikan_rs::{
    clubs::ClubSearchParams,
    common::enums::{
        clubs::{ClubOrder, ClubType},
        common::Sort,
    },
};
pub mod common;
use crate::common::macs::NamedTestJob;

ratelimited_test_runner!(run_ratelimited_tests, [
    make_client_test!(get_club_by_id, client, client.get_club_by_id(1)),
    make_client_test!(get_club_members, client, client.get_club_members(1, Some(1))),
    make_client_test!(get_club_staff, client, client.get_club_staff(1)),
    make_client_test!(get_club_relations, client, client.get_club_relations(1)),
    make_client_test!(get_club_search, client, client.get_club_search(
        Some(ClubSearchParams {
            q: Some("test".to_string()),
            page: Some(1),
            limit: Some(10),
            ..Default::default()
        })
    )),
    make_client_test!(get_club_search_with_category, client, client.get_club_search(
        Some(ClubSearchParams {
            page: Some(1),
            limit: Some(10),
            ..Default::default()
        })
    )),
    make_client_test!(get_club_search_with_order, client, client.get_club_search(
        Some(ClubSearchParams {
            page: Some(1),
            limit: Some(10),
            order_by: Some(ClubOrder::MembersCount),
            sort: Some(Sort::Desc),
            ..Default::default()
        })
    )),
    make_client_test!(get_club_search_with_letter, client, 
        client.get_club_search(
        Some(ClubSearchParams {
                page: Some(1),
                limit: Some(10),
                type_: Some(ClubType::Public),
                letter: Some("A".to_string()),
                ..Default::default()
            })
        )
    ),
]);

// #[tokio::test]
// #[serial]
// async fn get_nonexistent_club() {
//     let client = JikanClient::new();
//     let result = client.get_club_by_id(999999999).await;
//     assert!(matches!(result, Err(JikanError::NotFound)));
//     wait_between_tests().await;
// }

// #[tokio::test]
// #[serial]
// async fn get_nonexistent_club_members() {
//     let client = JikanClient::new();
//     let result = client.get_club_members(999999999, Some(1)).await;
//     assert!(matches!(result, Err(JikanError::NotFound)));
//     wait_between_tests().await;
// }

// #[tokio::test]
// #[serial]
// async fn get_nonexistent_club_staff() {
//     let client = JikanClient::new();
//     let result = client.get_club_staff(999999999).await;
//     assert!(matches!(result, Err(JikanError::NotFound)));
//     wait_between_tests().await;
// }
