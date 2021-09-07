use crate::post_repo::PostRepository;
use actix_web::web::{Data, Path};
use actix_web::{delete, get, post, put, HttpResponse, Responder};
use log::debug;

#[get("/post/{id}")]
pub async fn handle_get(path: Path<(u32,)>, repo: Data<PostRepository>) -> impl Responder {
    let id:i64 = path.into_inner().0.into();
    debug!("get {}", id);
    match repo.retrieve(id) {
        Some(post) => HttpResponse::Ok().json(post),
        None => HttpResponse::NotFound().body("not found"),
    }
}

#[post("/post/{id}")]
pub async fn handle_post(path: Path<(u32,)>, repo: Data<PostRepository>) -> impl Responder {
    let id = path.into_inner().0;
    debug!("post {}", id);
    HttpResponse::Ok().body("post")
}

#[put("/post")]
pub async fn handle_put(path: Path<(u32,)>, repo: Data<PostRepository>) -> impl Responder {
    let id = path.into_inner().0;
    debug!("put {}", id);
    HttpResponse::Ok().body("post")
}

#[delete("/post")]
pub async fn handle_delete(path: Path<(u32,)>, repo: Data<PostRepository>) -> impl Responder {
    HttpResponse::Ok().body("delete")
}
