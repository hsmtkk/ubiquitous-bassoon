use actix_web::{get, post, put, delete, HttpResponse, Responder};

#[get("/post")]
pub async fn handle_get() -> impl Responder {
    HttpResponse::Ok().body("get")
}

#[post("/post")]
pub async fn handle_post() -> impl Responder {
    HttpResponse::Ok().body("post")
}

#[put("/post")]
pub async fn handle_put() -> impl Responder {
    HttpResponse::Ok().body("put")
}

#[delete("/post")]
pub async fn handle_delete() -> impl Responder {
    HttpResponse::Ok().body("delete")
}
