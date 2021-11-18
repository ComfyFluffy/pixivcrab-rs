use super::{user::User, *};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Response {
    pub illusts: Vec<Illust>,
    pub next_url: Option<String>,
}
crate::impl_next_url!(Response);

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Illust {
    pub id: i64,
    pub title: String,
    pub r#type: String,
    pub image_urls: ImageURLs,
    pub caption: String,
    pub restrict: i64,
    pub user: User,
    pub tags: Vec<Tag>,
    pub tools: Vec<String>,
    pub create_date: DateTime<Utc>,
    pub page_count: i64,
    pub width: i64,
    pub height: i64,
    pub sanity_level: i64,
    pub x_restrict: i64,
    pub series: Option<Series>,
    pub meta_single_page: MetaSinglePage,
    pub meta_pages: Vec<MetaPage>,
    pub total_view: i64,
    pub total_bookmarks: i64,
    pub is_bookmarked: bool,
    pub visible: bool,
    pub is_muted: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MetaPage {
    pub image_urls: ImageURLs,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct MetaSinglePage {
    pub original_image_url: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct UgoiraResponse {
    pub ugoira_metadata: UgoiraMetadata,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct UgoiraMetadata {
    pub zip_urls: ZipUrls,
    pub frames: Vec<Frame>,
}
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Frame {
    pub file: String,
    pub delay: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ZipUrls {
    pub medium: String,
}
