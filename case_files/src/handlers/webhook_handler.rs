use actix_web::{ post, HttpResponse, Responder, Error, web };

use crate::models::update_model::Update;

#[post("/hook/updates")]
pub async fn handle_updates_webhook(update: web::Json<Update>) -> Result<impl Responder, Error>{
    Ok(HttpResponse::Ok().body(String::from("Hello")))
}