use std::{fmt::Debug, marker::PhantomData};

use futures::lock::Mutex;

use chrono::{DateTime, Duration, Utc};
use models::{auth, illust, novel, user};
use reqwest::{header::HeaderMap, ClientBuilder};
use serde::de::DeserializeOwned;
use snafu::ResultExt;
pub mod error;
pub mod models;

type Result<T> = std::result::Result<T, error::Error>;

#[derive(Debug, Clone)]
pub enum AuthMethod {
    RefreshToken(String),
}

pub trait NextUrl {
    fn next_url(&self) -> Option<&str>;
}

macro_rules! impl_next_url {
    ($t:ident) => {
        impl NextUrl for $t {
            fn next_url(&self) -> Option<&str> {
                self.next_url.as_ref().map(|x| x.as_str())
            }
        }
    };
}
pub(crate) use impl_next_url;

#[derive(Debug)]
pub struct Pager<'a, T>
where
    T: DeserializeOwned + NextUrl + Debug,
{
    app_api: &'a AppAPI,
    next_url: Option<String>,
    response_type: PhantomData<T>,
}

impl<T: DeserializeOwned + NextUrl + Debug> Pager<'_, T> {
    pub async fn next(&mut self) -> Result<Option<T>> {
        match &self.next_url {
            Some(url) => {
                let r = self
                    .app_api
                    .send_authorized(self.app_api.client.get(url))
                    .await?;
                match self.app_api.parse_json::<T>(r).await {
                    Ok(r) => {
                        self.next_url = r.next_url().map(|x| x.to_string());
                        Ok(Some(r))
                    }
                    Err(e) => Err(e),
                }
            }
            None => Ok(None),
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

#[derive(Debug)]
pub struct AppAPI {
    pub client: reqwest::Client,
    pub auth_method: AuthMethod,

    pub base_url: String,

    pub hash_secret: String,
    pub device_token: String,
    pub client_id: String,
    pub client_secret: String,

    pub auth_url: String,
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

pub fn default_headers(hash_secret: &str) -> HeaderMap {
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

impl AppAPI {
    pub fn new(
        auth_method: AuthMethod,
        language: &str,
        client_builder: ClientBuilder,
    ) -> Result<Self> {
        let mut base_headers = HeaderMap::new();
        base_headers.insert(
            "Accept-Language",
            language.parse().context(error::HeaderParse)?,
        );
        base_headers.insert("App-OS", "ios".parse().unwrap());
        base_headers.insert("App-OS-Version", "12.4.5".parse().unwrap());
        base_headers.insert("App-Version", "7.8.16".parse().unwrap());

        Ok(Self {
            client: client_builder
                .user_agent("PixivIOSApp/7.8.16 (iOS 12.4.5; iPhone7,2)")
                .timeout(std::time::Duration::from_secs(15))
                .default_headers(base_headers)
                .cookie_store(true)
                .build()
                .context(error::HTTP)?,
            base_url: "https://app-api.pixiv.net".to_string(),
            auth: Mutex::new(Auth::default()),
            auth_url: "https://oauth.secure.pixiv.net/auth/token".to_string(),
            auth_method,
            client_id: "KzEZED7aC0vird8jWyHM38mXjNTY".to_string(),
            client_secret: "W9JZoJe00qPvJsiyCGT3CCtC6ZUtdpKpzMbNlUGP".to_string(),
            device_token: "ec731472f8db58afe8588cbba92d5846".to_string(),
            hash_secret: "28c1fdd170a5204386cb1313c7077b34f83e4aaf4aa829ce78c231e05b0bae2c"
                .to_string(),
        })
    }

    pub fn headers(&self) -> HeaderMap {
        default_headers(&self.hash_secret)
    }

    pub async fn auth(&self) -> Result<AuthResult> {
        let mut auth = self.auth.lock().await;
        if !auth.access_token_expired() {
            return Ok(AuthResult {
                // Because the token is not expired, access_token and refresh_token are considered set here.
                access_token: auth.access_token.clone().unwrap(),
                refresh_token: auth.refresh_token.clone().unwrap(),
                user: auth.user.clone().unwrap(),
            });
        }
        let mut form = auth::Form {
            client_id: &self.client_id,
            client_secret: &self.client_secret,
            device_token: &self.device_token,
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
            match &self.auth_method {
                AuthMethod::RefreshToken(r) => {
                    form.grant_type = Some("refresh_token");
                    form.refresh_token = Some(r);
                }
            }
        }
        let r = self
            .client
            .post(&self.auth_url)
            .headers(self.headers())
            .form(&form)
            .send()
            .await
            .context(error::HTTP)?
            .json::<auth::Response>()
            .await
            .context(error::HTTP)?
            .response;

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

        request.headers(headers).send().await.context(error::HTTP)
    }

    async fn parse_json<T>(&self, response: reqwest::Response) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let status = response.status();
        if status.is_success() {
            response.json::<T>().await.context(error::HTTP)
        } else {
            error::PixivStatusCode { code: status }.fail()
        }
    }

    pub fn illust_bookmarks<'a>(
        &'a self,
        user_id: &str,
        private: bool,
    ) -> Pager<'a, illust::Response> {
        Pager {
            app_api: self,
            next_url: Some(format!(
                "{}/v1/user/bookmarks/illust?restrict={}&user_id={}",
                self.base_url,
                restrict(private),
                user_id
            )),
            response_type: PhantomData,
        }
    }

    pub fn illust_uploads<'a>(&'a self, user_id: &str) -> Pager<'a, illust::Response> {
        Pager {
            app_api: self,
            next_url: Some(format!(
                "{}/v1/user/illusts?user_id={}",
                self.base_url, user_id
            )),
            response_type: PhantomData,
        }
    }

    pub fn novel_bookmarks<'a>(
        &'a self,
        user_id: &str,
        private: bool,
    ) -> Pager<'a, novel::Response> {
        Pager {
            app_api: self,
            next_url: Some(format!(
                "{}/v1/user/bookmarks/novel?restrict={}&user_id={}",
                self.base_url,
                restrict(private),
                user_id
            )),
            response_type: PhantomData,
        }
    }

    pub fn novel_uploads<'a>(&'a self, user_id: &str) -> Pager<'a, novel::Response> {
        Pager {
            app_api: self,
            next_url: Some(format!(
                "{}/v1/user/novel?user_id={}",
                self.base_url, user_id
            )),
            response_type: PhantomData,
        }
    }

    pub async fn user_detail<'a>(&'a self, user_id: &str) -> Result<user::Response> {
        self.parse_json(
            self.send_authorized(
                self.client
                    .get(format!("{}/v1/user/detail", self.base_url,))
                    .query(&[("user_id", user_id)]),
            )
            .await?,
        )
        .await
    }

    pub async fn novel_text<'a>(&'a self, novel_id: &str) -> Result<novel::NovelTextResponse> {
        self.parse_json(
            self.send_authorized(
                self.client
                    .get(format!("{}/v1/novel/text", self.base_url))
                    .query(&[("novel_id", novel_id)]),
            )
            .await?,
        )
        .await
    }

    pub async fn ugoira_metadata<'a>(&'a self, illust_id: &str) -> Result<illust::UgoiraResponse> {
        self.parse_json(
            self.send_authorized(
                self.client
                    .get(format!("{}/v1/ugoira/metadata", self.base_url))
                    .query(&[("illust_id", illust_id)]),
            )
            .await?,
        )
        .await
    }
}
