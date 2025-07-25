use std::collections::HashMap;

use super::{user::User, *};
use serde_json::Value;
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
    pub is_original: Option<bool>,
    pub novel_ai_type: Option<i64>,
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

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WebViewNovelResponse {
    pub title: String,
    pub user_id: String,
    pub text: String,
    pub tags: Vec<String>,
    pub series_title: Option<String>,
    pub series_id: Option<String>,
    pub rating: Rating,
    pub is_original: bool,
    pub images: ArrayOrMap<Image>,
    pub id: String,
    pub cover_url: String,
    pub cdate: String,
    pub caption: String,
    pub ai_type: i64,
    pub series_is_watched: Option<bool>,
    pub series_navigation: Option<SeriesNavigation>,
    pub marker: Option<Value>,
    pub seasonal_effect_animation_urls: Option<Value>,
    pub replaceable_item_ids: Option<Vec<Value>>,
    pub glossary_items: Vec<Value>,
    pub illusts: ArrayOrMap<Value>,
    pub event_banners: Option<Value>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum ArrayOrMap<V> {
    Array(Vec<V>),
    Map(HashMap<String, V>),
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    pub novel_image_id: String,
    pub sl: String,
    pub urls: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub struct Rating {
    pub bookmark: u64,
    pub like: u64,
    pub view: u64,
}
#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SeriesNavigation {
    pub next_novel: Option<NovelPrevNext>,
    pub prev_novel: Option<NovelPrevNext>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NovelPrevNext {
    pub content_order: String,
    pub cover_url: String,
    pub id: u64,
    pub title: String,
    pub viewable: bool,
    pub viewable_message: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct NovelSeriesDetail1 {
    pub caption: String,
    pub content_count: i64,
    pub display_text: String,
    pub id: i64,
    pub is_concluded: bool,
    pub is_original: bool,
    pub novel_ai_type: i64,
    pub title: String,
    pub total_character_count: i64,
    pub user: User,
    pub watchlist_added: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct SeriesDetail {
    pub next_url: Option<String>,
    pub novel_series_detail: NovelSeriesDetail1,
    pub novel_series_first_novel: Novel,
    pub novel_series_latest_novel: Novel,
    pub novels: Vec<Novel>,
}
