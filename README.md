# pixivcrab

A pixiv AppAPI in Rust.

## Example

```rust
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
```

## Get Tokens

There is a script `pixiv_auth.py` in the project root. Use it to login and get access token & refresh token.

Usage:
```
# To login
$ python ./pixiv_auth.py login
# Here the script opens your browser and ask you to signin.
# Open the dev tools and navigate to Network tab, and complete the auth.
# You should be able to find a request `pixiv://account/login?code=...&via=login`.
# Copy the value of `code` and paste it into the terminal.
code: <code_found_in_dev_tools>

# To use refresh token to get access token
$ python ./pixiv_auth.py refresh <your_refresh_token>
```
