use miniprogram::api::App;

#[tokio::main]
async fn main() {
    let app = App::init().await;

    let listener = tokio::net::TcpListener::bind(app.get_listen())
        .await
        .unwrap();

    axum::serve(listener, app.get_router().clone())
        .await
        .unwrap();
}
