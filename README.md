# pixivcrab

A pixiv AppAPI bindings in Rust.

## Example

```rust
use pixivcrab::{AppAPI, AuthMethod};
use reqwest::Proxy;

#[tokio::main]
async fn main() {
    let client_builder =
        reqwest::Client::builder().proxy(Proxy::http("http://127.0.0.1:8080").unwrap());
    let api = AppAPI::new(
        AuthMethod::RefreshToken("refresh_token12345".to_string()),
        "en",
        client_builder,
    )
    .unwrap();
    let mut pager = api.illust_bookmarks("123456", false);
    while let Some(r) = pager.next().await.unwrap() {
        for i in r.illusts {
            println!("{} {:?}", i.title, i.tags);
        }
    }
}
```
