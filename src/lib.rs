pub mod routes;

use axum::{
    routing::{get, post},
    Router,
    extract::{Request, Path}
};
use tokio::net::TcpListener;

pub async fn run(listener: TcpListener) {
    // build our application with a single route
    let app = Router::new()
        .route("/health_check", get(routes::health_check))
        .route("/subscriptions", post(routes::subscribe));

    // run our app with hyper, listening globally on port 3000
    axum::serve(listener, app).await.unwrap();
}