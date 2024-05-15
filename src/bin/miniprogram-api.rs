use tracing::info;
use wechat::miniprogram_api::App;

#[tokio::main]
async fn main() {
    let app = App::init().await;

    let listener = tokio::net::TcpListener::bind(app.get_listen())
        .await
        .unwrap();

    info!("Listening on {}", app.get_listen());

    axum::serve(listener, app.get_router().clone())
        .await
        .unwrap();
}
