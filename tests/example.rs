use pixivcrab::{AppApi, AppApiConfig, AuthMethod};
use reqwest::ClientBuilder;
use std::env::var;

#[tokio::test]
async fn example() {
    let mut config = AppApiConfig::default();
    config.set_language("en-us").unwrap();
    let api = AppApi::new_with_config(
        AuthMethod::RefreshToken(var("PIXIV_REFRESH_TOKEN").unwrap()),
        ClientBuilder::new(),
        config,
    )
    .unwrap();
    let user = api.user_detail("123456").await.unwrap();
    println!("{:?}", user);
    let mut pager = api.illust_bookmarks("123456", false);
    while let Some(r) = pager.next().await.unwrap() {
        for i in r.illusts {
            println!("{} {:?}", i.title, i.tags);
        }
    }
}
