use super::{user::User, *};
use serde_with::{serde_as, DefaultOnError};

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Response {
    pub novels: Vec<Novel>,
    pub next_url: Option<String>,
}
crate::impl_next_url!(Response);

#[serde_as]
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Novel {
    pub id: i64,
    pub title: String,
    pub caption: String,
    pub restrict: i32,
    pub x_restrict: i32,
    pub image_urls: ImageUrls,
    pub create_date: DateTime<Utc>,
    pub tags: Vec<Tag>,
    pub page_count: i32,
    pub text_length: i32,
    pub user: User,
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub series: Option<Series>, // `series` can be {} (empty object)
    pub is_bookmarked: bool,
    pub total_bookmarks: i32,
    pub total_view: i32,
    pub visible: bool,
    pub total_comments: i32,
    pub is_muted: bool,
    pub is_mypixiv_only: bool,
    pub is_x_restricted: bool,
}

#[serde_as]
#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct NovelTextResponse {
    // novel_marker: NovelMarker,
    pub novel_text: String,
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub series_prev: Option<Novel>,
    #[serde_as(deserialize_as = "DefaultOnError")]
    pub series_next: Option<Novel>,
}
