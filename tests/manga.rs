mod common;
use crate::common::macs::NamedTestJob;

ratelimited_test_runner!(
    run_ratelimited_tests,
    [
        make_client_test!(get_manga, client, client.get_manga(1)),
        make_client_test!(get_manga_full, client, client.get_manga_full(1)),
        make_client_test!(get_manga_characters, client, client.get_manga_characters(1)),
        make_client_test!(get_manga_news, client, client.get_manga_news(1, Some(1))),
        make_client_test!(get_manga_forum, client, client.get_manga_forum(1, None)),
        make_client_test!(get_manga_pictures, client, client.get_manga_pictures(1)),
        make_client_test!(get_manga_statistics, client, client.get_manga_statistics(1)),
        make_client_test!(get_manga_moreinfo, client, client.get_manga_moreinfo(2)),
        make_client_test!(
            get_manga_recommendations,
            client,
            client.get_manga_recommendations(1)
        ),
        make_client_test!(
            get_manga_userupdates,
            client,
            client.get_manga_userupdates(1, Some(1))
        ),
        make_client_test!(
            get_manga_reviews,
            client,
            client.get_manga_reviews(1, Some(1), None, None)
        ),
        make_client_test!(get_manga_relations, client, client.get_manga_relations(1)),
        make_client_test!(get_manga_external, client, client.get_manga_external(1)),
    ]
);

// #[tokio::test]
// #[serial]
// async fn get_nonexistent_manga() {
//     let client = JikanClient::new();
//     let result = client.get_manga(999999999).await;
//     assert!(matches!(result, Err(JikanError::NotFound)));
//     wait_between_tests().await;
// }
