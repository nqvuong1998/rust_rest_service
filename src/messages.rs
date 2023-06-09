use crate::db_models::{User, Post};
use actix::Message;
use diesel::QueryResult;

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct CreateUserMessage {
  pub username: String,
  pub display_name: Option<String>,
  pub about_me: Option<String>,
  pub description: Option<String>,
  pub avatar: Option<String>
}

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct UpdateUserMessage {
  pub user_id: i32,
  pub display_name: Option<String>,
  pub about_me: Option<String>,
  pub description: Option<String>,
  pub avatar: Option<String>
}

#[derive(Message)]
#[rtype(result = "QueryResult<User>")]
pub struct GetUserMessage {
  pub user_id: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Post>")]
pub struct CreatePostMessage {
  pub author: i32,
  pub blocks: Option<String>,
  pub status: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Post>")]
pub struct UpdatePostMessage {
  pub post_id: i32,
  pub author: i32,
  pub blocks: Option<String>,
  pub status: String,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Post>")]
pub struct DeletePostMessage {
  pub post_id: i32,
  pub author: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Post>")]
pub struct GetPostMessage {
  pub post_id: i32,
  pub author: i32,
}

#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Post>>")]
pub struct GetPostsMessage {
  pub author: i32,
}