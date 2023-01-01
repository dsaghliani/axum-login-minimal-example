use std::net::SocketAddr;

#[tokio::main]
#[allow(clippy::unwrap_used)]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = axum_login_mvp::build_router();
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));

    tracing::info!("Listening on {address:?}.");

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
