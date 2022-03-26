# pixivcrab

A pixiv AppAPI in Rust.

## Example

```rust
use pixivcrab::{AppApi, AuthMethod};
use futures::TryStreamExt;

#[tokio::main]
async fn main() {
    let api = AppApi::new(
        AuthMethod::RefreshToken("your_refresh_token".to_string()),
        "en",
        reqwest::Client::builder(),
    )
    .unwrap();
    let user = api.user_detail("123456").await.unwrap();
    println!("{:?}", user);
    let mut pager = api.illust_bookmarks("123456", false);
    // `Pager` implements `futures::Stream`.
    // Import `futures::TryStreamExt` to use `try_next` method.
    while let Some(r) = pager.try_next().await.unwrap() {
        for i in r.illusts {
            println!("{} {:?}", i.title, i.tags);
        }
    }
}
```
