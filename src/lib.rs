//! A pixiv AppAPI in Rust.
//!
//! # Example
//!
//! ```no_run
//! use pixivcrab::{AppApi, AuthMethod};
//! use futures::TryStreamExt;
//!
//! #[tokio::main]
//! async fn main() {
//!     let api = AppApi::new(
//!         AuthMethod::RefreshToken("your_refresh_token".to_string()),
//!         "en",
//!         reqwest::Client::builder(),
//!     )
//!     .unwrap();
//!     let user = api.user_detail("123456").await.unwrap();
//!     println!("{:?}", user);
//!     let mut pager = api.illust_bookmarks("123456", false);
//!     // `Pager` implements `futures::Stream`.
//!     // Import `futures::TryStreamExt` to use `try_next` method.
//!     while let Some(r) = pager.try_next().await.unwrap() {
//!         for i in r.illusts {
//!             println!("{} {:?}", i.title, i.tags);
//!         }
//!     }
//! }
//! ```

use chrono::{DateTime, Duration, Utc};
use futures::lock::Mutex;
use models::{auth, illust, novel, user};
use reqwest::{
    header::{HeaderMap, InvalidHeaderValue},
    ClientBuilder,
};
use serde::de::DeserializeOwned;
use snafu::ResultExt;
use std::{fmt::Debug, marker::PhantomData, sync::Arc};

pub mod error;
pub mod models;

type Result<T> = std::result::Result<T, error::Error>;

#[derive(Debug, Clone)]
pub enum AuthMethod {
    RefreshToken(String),
}

pub trait NextUrl {
    fn next_url(&self) -> Option<String>;
}

macro_rules! impl_next_url {
    ($t:ident) => {
        impl NextUrl for $t {
            fn next_url(&self) -> Option<String> {
                self.next_url.clone()
            }
        }
    };
}
pub(crate) use impl_next_url;

/// A [`Pager`] streams the results from `next_url`.
/// It iterates over the pages until the `next_url` is None.
///
/// ```no_run
/// use pixivcrab::{AppApi, AuthMethod};
/// use futures::TryStreamExt;
///
/// #[tokio::main]
/// async fn main() {
///     let api = AppApi::new(
///         AuthMethod::RefreshToken("your_refresh_token".to_string()),
///         "en",
///         reqwest::Client::builder(),
///     )
///     .unwrap();
///     let mut pager = api.illust_bookmarks("123456", false);
///     while let Some(r) = pager.try_next().await.unwrap() {
///         for i in r.illusts {
///             println!("{} {:?}", i.title, i.tags);
///         }
///     }
/// }
/// ```
#[derive(Debug)]
pub struct Pager<T>
where
    T: DeserializeOwned + NextUrl + Send,
{
    pub next_url: Option<String>,
    app_api: AppApi,
    _response_type: PhantomData<fn() -> T>,
}

impl<T> Pager<T>
where
    T: DeserializeOwned + NextUrl + Send,
{
    fn new(app_api: AppApi, url: String) -> Self {
        Self {
            app_api,
            next_url: Some(url),
            _response_type: PhantomData,
        }
    }

    pub async fn next(&mut self) -> Result<Option<T>> {
        if let Some(ref url) = self.next_url {
            let r = self
                .app_api
                .send_authorized(self.app_api.0.client.get(url))
                .await?;
            let r = self.app_api.parse_json::<T>(r).await?;
            self.next_url = r.next_url();
            Ok(Some(r))
        } else {
            Ok(None)
        }
    }
}

fn restrict(private: bool) -> &'static str {
    if private {
        "private"
    } else {
        "public"
    }
}

#[derive(Debug, Clone)]
pub struct AppApi(Arc<AppApiInner>);

#[derive(Debug, Clone)]
pub struct AppApiConfig {
    pub base_url: String,
    pub hash_secret: String,
    pub device_token: String,
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub base_headers: HeaderMap,
    pub timeout: std::time::Duration,
    pub skip_config_client: bool,
    pub user_agent: String,
}

impl Default for AppApiConfig {
    fn default() -> Self {
        let mut base_headers = HeaderMap::new();
        base_headers.insert("Accept-Language", "en".parse().unwrap());
        base_headers.insert("App-OS", "ios".parse().unwrap());
        base_headers.insert("App-OS-Version", "12.4.5".parse().unwrap());
        base_headers.insert("App-Version", "7.8.16".parse().unwrap());

        Self {
            base_url: "https://app-api.pixiv.net".to_string(),
            auth_url: "https://oauth.secure.pixiv.net/auth/token".to_string(),
            client_id: "KzEZED7aC0vird8jWyHM38mXjNTY".to_string(),
            client_secret: "W9JZoJe00qPvJsiyCGT3CCtC6ZUtdpKpzMbNlUGP".to_string(),
            device_token: "ec731472f8db58afe8588cbba92d5846".to_string(),
            hash_secret: "28c1fdd170a5204386cb1313c7077b34f83e4aaf4aa829ce78c231e05b0bae2c"
                .to_string(),
            base_headers,
            timeout: std::time::Duration::from_secs(15),
            skip_config_client: false,
            user_agent: "PixivIOSApp/7.8.16 (iOS 12.4.5; iPhone7,2)".to_string(),
        }
    }
}

impl AppApiConfig {
    /// Set language used in reguest.
    ///
    /// # Errors
    ///
    /// Error if `language` is of invalid header value.
    pub fn set_language(&mut self, language: &str) -> std::result::Result<(), InvalidHeaderValue> {
        let v = language.parse()?;
        self.base_headers.insert("Accept-Language", v);
        Ok(())
    }
}

#[derive(Debug)]
struct AppApiInner {
    client: reqwest::Client,
    auth_method: AuthMethod,
    config: AppApiConfig,
    auth: Mutex<Auth>,
}

#[derive(Debug, Clone, Default)]
struct Auth {
    refresh_token: Option<String>,
    access_token: Option<String>,
    access_token_expires_at: Option<DateTime<Utc>>,
    access_token_expires_delta: Option<Duration>,
    user: Option<models::auth::User>,
}

#[derive(Debug, Clone)]
pub struct AuthResult {
    pub access_token: String,
    pub refresh_token: String,
    pub user: models::auth::User,
}

impl Auth {
    fn access_token_expired(&self) -> bool {
        if let Some(expires_at) = self.access_token_expires_at {
            if let Some(expires_delta) = self.access_token_expires_delta {
                return Utc::now() > expires_at - expires_delta;
            }
        }
        self.access_token.is_none()
    }
}

pub fn x_client_headers(hash_secret: &str) -> HeaderMap {
    let mut h = HeaderMap::new();
    let t = chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true);
    h.insert(
        "X-Client-Hash",
        format!("{:x}", md5::compute(t.clone() + hash_secret))
            .parse()
            .unwrap(),
    );
    h.insert("X-Client-Time", t.parse().unwrap());
    h
}

impl AppApi {
    pub fn default_client_builder() -> ClientBuilder {
        ClientBuilder::new().cookie_store(true)
    }

    /// Create new instance of `AppApi` with default config.
    pub fn new(auth_method: AuthMethod) -> Result<Self> {
        Self::new_with_config(
            auth_method,
            Self::default_client_builder(),
            AppApiConfig::default(),
        )
    }

    pub fn new_with_config(
        auth_method: AuthMethod,
        mut client_builder: ClientBuilder,
        config: AppApiConfig,
    ) -> Result<Self> {
        if !config.skip_config_client {
            client_builder = client_builder
                .user_agent(&config.user_agent)
                .timeout(config.timeout)
                .default_headers(config.base_headers.clone())
                .cookie_store(true);
        }
        Ok(Self(Arc::new(AppApiInner {
            client: client_builder.build().context(error::Http)?,
            auth: Mutex::new(Auth::default()),
            auth_method,
            config,
        })))
    }

    pub fn headers(&self) -> HeaderMap {
        x_client_headers(&self.0.config.hash_secret)
    }

    pub async fn auth(&self) -> Result<AuthResult> {
        let mut auth = self.0.auth.lock().await;
        if !auth.access_token_expired() {
            return Ok(AuthResult {
                // Because the token is not expired, access_token and refresh_token are considered set here.
                access_token: auth.access_token.clone().unwrap(),
                refresh_token: auth.refresh_token.clone().unwrap(),
                user: auth.user.clone().unwrap(),
            });
        }
        let mut form = auth::Form {
            client_id: &self.0.config.client_id,
            client_secret: &self.0.config.client_secret,
            device_token: &self.0.config.device_token,
            get_secure_url: true,
            include_policy: true,
            grant_type: None,
            username: None,
            password: None,
            refresh_token: None,
        };
        if let Some(ref r) = auth.refresh_token {
            form.grant_type = Some("refresh_token");
            form.refresh_token = Some(r);
        } else {
            match &self.0.auth_method {
                AuthMethod::RefreshToken(r) => {
                    form.grant_type = Some("refresh_token");
                    form.refresh_token = Some(r);
                }
            }
        }
        let resp = self
            .0
            .client
            .post(&self.0.config.auth_url)
            .headers(self.headers())
            .form(&form)
            .send()
            .await
            .context(error::Http)?;

        let r = self.parse_json::<auth::Response>(resp).await?.response;

        auth.refresh_token = Some(r.refresh_token.clone());
        auth.access_token = Some(r.access_token.clone());
        auth.access_token_expires_at = Some(Utc::now() + Duration::seconds(r.expires_in));
        auth.access_token_expires_delta = Some(Duration::seconds(r.expires_in / 100));
        auth.user = Some(r.user.clone());

        Ok(AuthResult {
            refresh_token: r.refresh_token,
            access_token: r.access_token,
            user: r.user,
        })
    }

    pub async fn send_authorized(
        &self,
        request: reqwest::RequestBuilder,
    ) -> Result<reqwest::Response> {
        let access_token = self.auth().await?.access_token;

        let mut headers = self.headers();
        headers.insert(
            "Authorization",
            format!("Bearer {}", access_token)
                .parse()
                .context(error::HeaderParse)?,
        );

        request.headers(headers).send().await.context(error::Http)
    }

    async fn parse_json<T>(&self, response: reqwest::Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let status = response.status();
        if status.is_success() {
            let body = response.bytes().await.context(error::Http)?;
            serde_json::from_slice(&body).context(error::UnexpectedJson)
        } else {
            let text = response.text().await.context(error::Http)?;
            error::UnexpectedStatus { status, text }.fail()
        }
    }

    pub fn illust_bookmarks(&self, user_id: &str, private: bool) -> Pager<illust::Response> {
        Pager::new(
            self.clone(),
            format!(
                "{}/v1/user/bookmarks/illust?restrict={}&user_id={}",
                self.0.config.base_url,
                restrict(private),
                user_id
            ),
        )
    }

    pub fn illust_uploads(&self, user_id: &str) -> Pager<illust::Response> {
        Pager::new(
            self.clone(),
            format!(
                "{}/v1/user/illusts?user_id={}",
                self.0.config.base_url, user_id
            ),
        )
    }

    pub fn novel_bookmarks(&self, user_id: &str, private: bool) -> Pager<novel::Response> {
        Pager::new(
            self.clone(),
            format!(
                "{}/v1/user/bookmarks/novel?restrict={}&user_id={}",
                self.0.config.base_url,
                restrict(private),
                user_id
            ),
        )
    }

    pub fn novel_uploads(&self, user_id: &str) -> Pager<novel::Response> {
        Pager::new(
            self.clone(),
            format!(
                "{}/v1/user/novels?user_id={}",
                self.0.config.base_url, user_id
            ),
        )
    }

    pub async fn user_detail(&self, user_id: &str) -> Result<user::Response> {
        self.parse_json(
            self.send_authorized(
                self.0
                    .client
                    .get(format!("{}/v1/user/detail", self.0.config.base_url,))
                    .query(&[("user_id", user_id)]),
            )
            .await?,
        )
        .await
    }

    pub async fn novel_text<'a>(&self, novel_id: &str) -> Result<novel::NovelTextResponse> {
        self.parse_json(
            self.send_authorized(
                self.0
                    .client
                    .get(format!("{}/v1/novel/text", self.0.config.base_url))
                    .query(&[("novel_id", novel_id)]),
            )
            .await?,
        )
        .await
    }

    pub async fn ugoira_metadata<'a>(&self, illust_id: &str) -> Result<illust::UgoiraResponse> {
        self.parse_json(
            self.send_authorized(
                self.0
                    .client
                    .get(format!("{}/v1/ugoira/metadata", self.0.config.base_url))
                    .query(&[("illust_id", illust_id)]),
            )
            .await?,
        )
        .await
    }
}
