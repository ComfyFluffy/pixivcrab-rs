pub mod auth;
pub mod comment;
pub mod illust;
pub mod novel;
pub mod user;

use crate::NextUrl;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct ImageURLs {
    pub square_medium: Option<String>,
    pub medium: Option<String>,
    pub large: Option<String>,
    pub original: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Tag {
    pub name: String,
    pub translated_name: Option<String>,
    pub added_by_uploaded_user: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Series {
    pub id: i64,
    pub title: String,
}
