mod config;
mod handlers;
mod models;

use std::{ env, process };

use actix_web::{ HttpServer, middleware::Logger, App };
use config::Config;
use dotenvy::dotenv;

// Entry point

#[actix_web::main]
async fn main() {
    dotenv().ok();

    let tg_api_key = env::var("TG_API_KEY").unwrap_or_else(|_| {
        println!("Add Telegram API key as TG_API_KEY to .env file");
        process::exit(-1);
    });

    let config = Config::build(tg_api_key, 5000);

    match run(config).await {
        Ok(_) => println!("Successful run"),
        Err(_) => println!("Something went wrong"),
    }
}

async fn run(config: Config) -> Result<(), actix_web::Error> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new().wrap(logger).service(handlers::index_handler::index)
    })
        .bind(("127.0.0.1", config.port))?
        .run().await
        .map(|_| Ok(()))?
}
