use actix_web::*;

mod routes;
use routes::{ping};

#[actix_web::main];

// RAII - Escopo 

async fn main() -> Result<(), std::io::Error>{
    let api = HttpServer::new( || {
        App::new()
        .route("/ping", web::get().to(ping))
    });

    let porta: i32 = 9001;
    let api = api.bind(format!("127.0.0.1:{}", porta)).expect("Não conseguiu logar")


    println!("Conectado com Sucessso {}", porta);

    api.run()
    .await  
    }
