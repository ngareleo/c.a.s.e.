mod config;
mod handlers;
mod models;
mod internal;

use std::{ env, process };

use actix_web::{ HttpServer, middleware::Logger, App, web, error, HttpResponse };
use config::Config;
use dotenvy::dotenv;
use internal::case::Case;

// Entry point

#[derive(Clone)]
struct AppState {
    case: Case,
}

#[actix_web::main]
async fn main() {
    dotenv().ok();

    let tg_api_key = env::var("TG_API_KEY").unwrap_or_else(|_| {
        println!("Add Telegram API key as TG_API_KEY to .env file");
        process::exit(-1);
    });

    let port = env::var("PORT").unwrap_or("5000".to_string()).parse().unwrap();
    let config = Config::build(tg_api_key.clone(), port);
    let appState = AppState {
        case: Case::new(tg_api_key),
    };

    match run(config, appState).await {
        Ok(_) => println!("Server shutdown gracefully🫡"),
        Err(_) => println!("⚠️Server crash"),
    }
}

async fn run(config: Config, state: AppState) -> Result<(), actix_web::Error> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    env_logger::init();

    let server = HttpServer::new(move || {
        let logger = Logger::default();
        let json_config = web::JsonConfig
            ::default()
            .limit(6000)
            .error_handler(|err, _req| {
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });

        App::new()
            .wrap(logger)
            .app_data(state.case.clone())
            .app_data(json_config)
            .service(handlers::index_handler::index)
            .service(handlers::webhook_handler::handle_updates_webhook)
    }).bind(("127.0.0.1", config.port));

    println!("Listening on http://localhost:{} !🚀", config.port);

    let _ = server.expect("").run().await;
    Ok(())
}
