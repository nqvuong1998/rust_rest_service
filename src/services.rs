use actix_web::{
    get, post,put,delete,
    web::{Data, Json, Path},
    Responder, HttpResponse,
};
use serde::Deserialize;
use crate::{
    messages::{GetUserMessage,CreatePostMessage,CreateUserMessage,UpdateUserMessage,UpdatePostMessage,DeletePostMessage,GetPostMessage,GetPostsMessage},
    AppState, DbActor
};
use actix::Addr;

#[derive(Deserialize)]
pub struct CreateUserBody {
    pub username: String,
    pub display_name: Option<String>,
    pub about_me: Option<String>,
    pub description: Option<String>,
    pub avatar: Option<String>
}

#[post("/users")]
pub async fn create_user(state: Data<AppState>, body: Json<CreateUserBody>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();
    
    match db.send(CreateUserMessage {
        username: body.username.to_string(),
        display_name: body.display_name.clone(),
        about_me: body.about_me.clone(),
        description: body.description.clone(),
        avatar: body.avatar.clone(),
    }).await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to create user"),
    }
}

#[derive(Deserialize)]
pub struct UpdateUserBody {
    pub display_name: Option<String>,
    pub about_me: Option<String>,
    pub description: Option<String>,
    pub avatar: Option<String>
}

#[put("/users/{user_id}")]
pub async fn update_user(state: Data<AppState>, path: Path<i32>, body: Json<UpdateUserBody>) -> impl Responder {
    let user_id: i32 = path.into_inner();

    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(UpdateUserMessage {
        user_id: user_id,
        display_name: body.display_name.clone(),
        about_me: body.about_me.clone(),
        description: body.description.clone(),
        avatar: body.avatar.clone(),
    }).await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to update user"),
    }
}

#[get("/users/{user_id}")]
pub async fn get_user(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let user_id: i32 = path.into_inner();

    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(GetUserMessage { user_id: user_id }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No found user"),
        _ => HttpResponse::InternalServerError().json("Failed to get user"),
    }
}

#[derive(Deserialize)]
pub struct CreatePostBody {
    pub blocks: Option<String>,
    pub status: String,
}

#[post("/users/{user_id}/posts")]
pub async fn create_post(state: Data<AppState>, path: Path<i32>, body: Json<CreatePostBody>) -> impl Responder {
    let user_id: i32 = path.into_inner();

    let db: Addr<DbActor> = state.as_ref().db.clone();

    let status = body.status.to_string().trim().to_uppercase();
    if !(status.eq ("PUBLISHED") || status.eq("DRAFT")){
        return HttpResponse::InternalServerError().json("Wrong status format");
    }
    
    match db.send(CreatePostMessage {
        author: user_id,
        blocks: body.blocks.clone(),
        status: status
    }).await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to create post"),
    }
}

#[derive(Deserialize)]
pub struct UpdatePostBody {
    pub blocks: Option<String>,
    pub status: String,
}

#[put("/users/{user_id}/posts/{post_id}")]
pub async fn update_post(state: Data<AppState>, path: Path<(i32, i32)>, body: Json<UpdatePostBody>) -> impl Responder {
    let data = path.into_inner();
    let user_id: i32 = data.0;
    let post_id: i32 = data.1;

    let db: Addr<DbActor> = state.as_ref().db.clone();

    let status = body.status.to_string().trim().to_uppercase();
    if !(status.eq ("PUBLISHED") || status.eq("DRAFT")){
        return HttpResponse::InternalServerError().json("Wrong status format");
    }
    
    match db.send(UpdatePostMessage {
        post_id: post_id,
        author: user_id,
        blocks: body.blocks.clone(),
        status: status
    }).await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to update post"),
    }
}

#[delete("/users/{user_id}/posts/{post_id}")]
pub async fn delete_post(state: Data<AppState>, path: Path<(i32, i32)>) -> impl Responder {
    let data = path.into_inner();
    let user_id: i32 = data.0;
    let post_id: i32 = data.1;

    let db: Addr<DbActor> = state.as_ref().db.clone();
    
    match db.send(DeletePostMessage {
        post_id: post_id,
        author: user_id,
    }).await
    {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        _ => HttpResponse::InternalServerError().json("Failed to delete post"),
    }
}

#[get("/users/{user_id}/posts/{post_id}")]
pub async fn get_post(state: Data<AppState>, path: Path<(i32, i32)>) -> impl Responder {
    let data = path.into_inner();
    let user_id: i32 = data.0;
    let post_id: i32 = data.1;

    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(GetPostMessage { post_id: post_id, author: user_id }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No found post"),
        _ => HttpResponse::InternalServerError().json("Failed to get post"),
    }
}

#[get("/users/{user_id}/posts")]
pub async fn get_posts(state: Data<AppState>, path: Path<i32>) -> impl Responder {
    let user_id = path.into_inner();

    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(GetPostsMessage { author: user_id }).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No found post"),
        _ => HttpResponse::InternalServerError().json("Failed to get post"),
    }
}
