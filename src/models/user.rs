use super::*;
use std::collections::BTreeMap;

pub type Workspace = BTreeMap<String, Option<String>>;

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Response {
    pub user: User,
    pub profile: Profile,
    pub profile_publicity: ProfilePublicity,
    pub workspace: Workspace,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub account: String,
    pub profile_image_urls: ProfileImageUrls,
    pub is_followed: Option<bool>,
    pub comment: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ProfileImageUrls {
    pub medium: String,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Profile {
    pub webpage: Option<String>,
    pub gender: String,
    pub birth: String,
    pub birth_day: String,
    pub birth_year: i32,
    pub region: Option<String>,
    pub address_id: i64,
    pub country_code: Option<String>,
    pub job: Option<String>,
    pub job_id: i64,
    pub total_follow_users: i32,
    pub total_mypixiv_users: i32,
    pub total_illusts: i32,
    pub total_manga: i32,
    pub total_novels: i32,
    pub total_illust_bookmarks_public: i32,
    pub total_illust_series: i32,
    pub total_novel_series: i32,
    pub background_image_url: Option<String>,
    pub twitter_account: Option<String>,
    pub twitter_url: Option<String>,
    pub pawoo_url: Option<String>,
    pub is_premium: bool,
    pub is_using_custom_profile_image: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ProfilePublicity {
    pub gender: String,
    pub region: String,
    pub birth_day: String,
    pub birth_year: String,
    pub job: String,
    pub pawoo: bool,
}
