use miniprogram::api::app::App;

#[tokio::main]
async fn main() {
    let app = App::new().await;

    axum::Server::bind(app.get_listen())
        .serve(app.get_router().clone().into_make_service())
        .await
        .unwrap();
}
