use crate::schema::{posts,users};
use diesel::{Insertable,AsChangeset};
use serde::Serialize;

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=users)]
pub struct CreateUser {
  pub username: String,
  pub display_name: Option<String>,
  pub about_me: Option<String>,
  pub description: Option<String>,
  pub avatar: Option<String>
}

#[derive(AsChangeset, Serialize, Clone)]
#[diesel(table_name=users)]
pub struct UpdateUser {
  pub display_name: Option<String>,
  pub about_me: Option<String>,
  pub description: Option<String>,
  pub avatar: Option<String>
}

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name=posts)]
pub struct CreatePost {
  pub author: i32,
  pub blocks: Option<String>,
  pub status: String,
}

#[derive(AsChangeset, Serialize, Clone)]
#[diesel(table_name=posts)]
pub struct UpdatePost {
  pub blocks: Option<String>,
  pub status: String,
}

#[derive(AsChangeset, Serialize, Clone)]
#[diesel(table_name=posts)]
pub struct DeletePost {
  pub status: String,
}
