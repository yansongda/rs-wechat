use miniprogram::api::app::App;

#[tokio::main]
async fn main() {
    let app = App::new();

    axum::Server::bind(&app.listen)
        .serve(app.router.into_make_service())
        .await
        .unwrap();
}
