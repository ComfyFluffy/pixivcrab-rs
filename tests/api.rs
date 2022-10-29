use std::env::var;

use pixivcrab::AppApi;

fn init() -> AppApi {
    dotenvy::dotenv().ok();
    AppApi::new(pixivcrab::AuthMethod::RefreshToken(
        var("PIXIV_REFRESH_TOKEN").unwrap(),
    ))
    .unwrap()
}

#[derive(Clone, Copy)]
struct Ptr(*mut i32);

unsafe impl Send for Ptr {}

#[tokio::test]
async fn test_auth() {
    init();
    let api = init();
    let auth = api.auth().await.unwrap();
    println!("{:?}", auth);
}
