use actix_web::*


pub async fn ping() -> HttpReponse {
    HttpReponse::Ok().body("conectado...")
}

