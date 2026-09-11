mod domain;
mod application;
mod infrastructure;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
dotenv().ok();
let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
infrastructure::establish_connection(&database_url);

HttpServer::new(|| {
    App::new()
       .service(web::resource("/transactions").route(web::post().to(infrastructure::create_transaction))
       .route(web::get().to(infrastructure::list_transactions))
       .route(web::put().to(infrastructure::update_transaction))
       .route(web::delete().to(infrastructure::delete_transaction)))
})
.bind("127.0.0.1:8080")?
.run()
.await
}