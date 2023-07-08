use actix_web::{ get, HttpResponse, Responder, Error };

#[get("/")]
pub async fn index() -> Result<impl Responder, Error> {
    Ok(HttpResponse::Ok().body(String::from("Hello")))
}
