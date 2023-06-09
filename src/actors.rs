use crate::db_models::{User, Post};
use crate::db_utils::DbActor;
use crate::schema::users::{dsl::*, id as user_id};
use crate::schema::posts::{dsl::*, id as post_id};
use crate::messages::{GetUserMessage,CreatePostMessage,CreateUserMessage,UpdateUserMessage,UpdatePostMessage,DeletePostMessage,GetPostMessage,GetPostsMessage};
use crate::insertables::{CreatePost,CreateUser,UpdateUser,UpdatePost,DeletePost};
use actix::Handler;
use diesel::{self, prelude::*};
use diesel::dsl::not;

impl Handler<CreateUserMessage> for DbActor {
  type Result = QueryResult<User>;

  fn handle(&mut self, msg: CreateUserMessage, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Create User: Unable to establish connection");

    let new_user = CreateUser {
      username: msg.username,
      display_name: msg.display_name,
      about_me: msg.about_me,
      description: msg.description,
      avatar: msg.avatar
    };

    diesel::insert_into(users)
      .values(new_user)
      .returning((
        user_id,
        username,
        display_name.nullable(),
        about_me.nullable(),
        description.nullable(),
        avatar.nullable()
      ))
      .get_result::<User>(&mut conn)
  }
}

impl Handler<UpdateUserMessage> for DbActor {
  type Result = QueryResult<User>;

  fn handle(&mut self, msg: UpdateUserMessage, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Update User: Unable to establish connection");

    let update_user = UpdateUser {
      display_name: msg.display_name,
      about_me: msg.about_me,
      description: msg.description,
      avatar: msg.avatar
    };

    diesel::update(users)
    .filter(user_id.eq(msg.user_id))
    .set::<UpdateUser>(update_user)
    .get_result::<User>(&mut conn)
  }
}

impl Handler<GetUserMessage> for DbActor {
  type Result = QueryResult<User>;

  fn handle(&mut self, msg: GetUserMessage, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Get User: Unable to establish connection");

    users.filter(user_id.eq(msg.user_id)).first::<User>(&mut conn)
  }
}

impl Handler<CreatePostMessage> for DbActor {
  type Result = QueryResult<Post>;

  fn handle(&mut self, msg: CreatePostMessage, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Create User: Unable to establish connection");

    let new_post = CreatePost {
      author: msg.author,
      blocks: msg.blocks,
      status: msg.status
    };

    diesel::insert_into(posts)
      .values(new_post)
      .returning((
        post_id,
        author,
        blocks.nullable(),
        created_at.nullable(),
        updated_at.nullable(),
        status
      ))
      .get_result::<Post>(&mut conn)
  }
}

impl Handler<UpdatePostMessage> for DbActor {
  type Result = QueryResult<Post>;

  fn handle(&mut self, msg: UpdatePostMessage, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Delete Post: Unable to establish connection");

    let update_post = UpdatePost {
      blocks: msg.blocks,
      status: msg.status
    };

    diesel::update(posts)
    .filter(post_id.eq(msg.post_id).and(author.eq(msg.author)))
    .set::<UpdatePost>(update_post)
    .get_result::<Post>(&mut conn)
  }
}

impl Handler<DeletePostMessage> for DbActor {
  type Result = QueryResult<Post>;

  fn handle(&mut self, msg: DeletePostMessage, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Get Post: Unable to establish connection");

    let delete_post = DeletePost {
      status: "DELETED".to_string()
    };

    diesel::update(posts)
    .filter(post_id.eq(msg.post_id).and(author.eq(msg.author)))
    .set::<DeletePost>(delete_post)
    .get_result::<Post>(&mut conn)
  }
}

impl Handler<GetPostMessage> for DbActor {
  type Result = QueryResult<Post>;

  fn handle(&mut self, msg: GetPostMessage, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Get Post: Unable to establish connection");

    posts.filter(post_id.eq(msg.post_id).and(author.eq(msg.author)).and(not(status.eq("DELETED")))).first::<Post>(&mut conn)
  }
}

impl Handler<GetPostsMessage> for DbActor {
  type Result = QueryResult<Vec<Post>>;

  fn handle(&mut self, msg: GetPostsMessage, _ctx: &mut Self::Context) -> Self::Result {
    let mut conn = self.0.get().expect("Get Post: Unable to establish connection");

    posts.filter(author.eq(msg.author).and(not(status.eq("DELETED")))).get_results::<Post>(&mut conn)
  }
}
