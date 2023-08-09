use actix_web::{ post, web };

use crate::{ models::update_model::Update, internal::case::Case };

#[post("/hook/updates")]
pub async fn handle_updates_webhook(update: web::Json<Update>, case: web::Data<Case>) -> String {
    let message = update.message.as_ref().unwrap();
    let text = format!("Message is {}", message.text.as_ref().unwrap()).to_string();
    println!("{text}");
    text
}
