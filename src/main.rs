use std::env;

use actix_web::{HttpServer, App, middleware::Logger};
use dotenvy::dotenv;


pub mod routes;
pub mod models;
pub mod handlers;


#[actix_rt::main]
async fn main() -> Result<(), std::io::Error>{
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();
    

    let server_url = env::var("SERVER_URL")
        .expect("SERVER_URL must be set");


    HttpServer::new(move || {

        App::new()
        .wrap(Logger::default())
        .configure(routes::routes_factory)

    })
    .bind(server_url)?
    .run()
    .await
}
