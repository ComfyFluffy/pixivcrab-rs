use super::{user::User, *};

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Response {
    pub illusts: Vec<Illust>,
    pub next_url: Option<String>,
}
crate::impl_next_url!(Response);

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Illust {
    pub id: i64,
    pub title: String,
    pub r#type: String,
    pub image_urls: ImageUrls,
    pub caption: String,
    pub restrict: i32,
    pub user: User,
    pub tags: Vec<Tag>,
    pub tools: Vec<String>,
    pub create_date: DateTime<Utc>,
    pub page_count: i32,
    pub width: i32,
    pub height: i32,
    pub sanity_level: i32,
    pub x_restrict: i32,
    pub series: Option<Series>,
    pub meta_single_page: MetaSinglePage,
    pub meta_pages: Vec<MetaPage>,
    pub total_view: i32,
    pub total_bookmarks: i32,
    pub is_bookmarked: bool,
    pub visible: bool,
    pub is_muted: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct MetaPage {
    pub image_urls: ImageUrls,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct MetaSinglePage {
    pub original_image_url: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct UgoiraResponse {
    pub ugoira_metadata: UgoiraMetadata,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct UgoiraMetadata {
    pub zip_urls: ZipUrls,
    pub frames: Vec<Frame>,
}
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Frame {
    pub file: String,
    pub delay: i32,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ZipUrls {
    pub medium: String,
}
