use super::{user::User, *};

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Response {
    pub comments: Vec<Comment>,
    pub next_url: Option<String>,
}
crate::impl_next_url!(Response);

#[derive(Clone, Debug, Default, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Comment {
    pub id: i64,
    pub comment: String,
    pub date: DateTime<Utc>,
    pub user: User,
    pub has_replies: bool,
}
