use actix_web::{web, App, HttpServer, Responder, http};
use actix_cors::Cors;
use dotenv::dotenv;

mod redis_adapter;
mod controller;


async fn greet() -> impl Responder {
    "Welcome to Lead-API!"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load the .env file
    dotenv().ok(); 

    // Initialize the leaderboard in Redis
    redis_adapter::init_leaderboard();

    // Start the Actix HTTP server
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin() // Specify the allowed origin
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"]) // Specify the allowed methods
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .wrap(cors)
            .route("/", web::get().to(greet))
            .route("/leaderboard", web::get().to(controller::fetch_top_leaderboard))
            .route("/leaderboard/{page}", web::get().to(controller::fetch_leaderboard))
            .route("/leaderboard_count", web::get().to(controller::fetch_leaderboard_count))
            .route("/leaderboard/{name}", web::delete().to(controller::delete_player))
            .route("/leaderboard/{name}/{score}", web::post().to(controller::add_player))
            .route("/leaderboard/{name}/{score}", web::put().to(controller::update_player))
            .route("/ws/", web::get().to(controller::new_socket))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
