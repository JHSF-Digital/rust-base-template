use axum::Router;
use tokio::net::TcpListener;

mod routes;
mod handlers;
mod middlewares;

use routes::health_check::health_routes;

#[tokio::main]
async fn main() {
    let app = Router::new().merge(health_routes());

    // run it
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();


    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}