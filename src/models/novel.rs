use super::{user::User, *};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Response {
    pub novels: Vec<Novel>,
    pub next_url: Option<String>,
}

impl NextUrl for Response {
    fn next_url(&self) -> Option<String> {
        self.next_url.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Novel {
    pub id: i64,
    pub title: String,
    pub caption: String,
    pub restrict: i64,
    pub x_restrict: i64,
    pub image_urls: ImageURLs,
    pub create_date: DateTime<Utc>,
    pub tags: Vec<Tag>,
    pub page_count: i64,
    pub text_length: i64,
    pub user: User,
    pub series: Series,
    pub is_bookmarked: bool,
    pub total_bookmarks: i64,
    pub total_view: i64,
    pub visible: bool,
    pub total_comments: i64,
    pub is_muted: bool,
    pub is_mypixiv_only: bool,
    pub is_x_restricted: bool,
}
