use super::*;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Response {
    pub response: AuthResponse,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct AuthResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
    pub scope: String,
    pub refresh_token: String,
    pub user: User,
    pub device_token: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct User {
    pub profile_image_urls: ProfileImageURLs,
    pub id: String,
    pub name: String,
    pub account: String,
    pub mail_address: String,
    pub is_premium: bool,
    pub x_restrict: i64,
    pub is_mail_authorized: bool,
    pub require_policy_agreement: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ProfileImageURLs {
    pub px_16x16: String,
    pub px_50x50: String,
    pub px_170x170: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Hash)]
pub struct Form<'a> {
    pub client_id: &'a str,
    pub client_secret: &'a str,
    pub device_token: &'a str,
    pub get_secure_url: bool,
    pub include_policy: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_type: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<&'a str>,
}
