use super::{user::User, *};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Response {
    pub comments: Vec<Comment>,
    pub next_url: Option<String>,
}

impl NextUrl for Response {
    fn next_url(&self) -> Option<String> {
        self.next_url.clone()
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Comment {
    pub id: i64,
    pub comment: String,
    pub date: DateTime<Utc>,
    pub user: User,
    pub has_replies: bool,
}
