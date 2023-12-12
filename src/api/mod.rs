use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use axum::routing::get;
use axum::Router;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::metadata::LevelFilter;
use tracing::Level;
use tracing_appender::non_blocking::{NonBlockingBuilder, WorkerGuard};
use tracing_subscriber::filter;
use tracing_subscriber::fmt::time::ChronoLocal;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use crate::config::Config;
use crate::model::result::Response;
use crate::repository::Pool;

mod extract;
mod middleware;
mod response;
mod routes;
mod v1;

pub struct App {
    _logger: WorkerGuard,
    listen: SocketAddr,
    router: Router,
}

impl App {
    pub async fn init() -> Self {
        Config::init();

        Pool::init().await;

        App {
            _logger: App::logger(),
            listen: App::listen(),
            router: App::router(),
        }
    }

    pub fn get_listen(&self) -> &SocketAddr {
        &self.listen
    }

    pub fn get_router(&self) -> &Router {
        &self.router
    }

    fn logger() -> WorkerGuard {
        let (non_blocking, guard) = NonBlockingBuilder::default().finish(std::io::stdout());

        tracing_subscriber::registry()
            .with(
                filter::Targets::new()
                    .with_target("sea_orm", Level::DEBUG)
                    .with_default(
                        LevelFilter::from_str(if Config::get::<bool>("bin.api.debug") {
                            "debug"
                        } else {
                            "info"
                        })
                        .unwrap(),
                    ),
            )
            .with(
                tracing_subscriber::fmt::layer()
                    .with_writer(non_blocking)
                    .with_timer(ChronoLocal::new("%Y-%m-%d %H:%M:%S%.6f".to_string())),
            )
            .init();

        guard
    }

    fn listen() -> SocketAddr {
        let listen = Config::get::<String>("bin.api.listen");
        let port = Config::get::<u16>("bin.api.port");

        SocketAddr::from((IpAddr::from_str(listen.as_str()).unwrap(), port))
    }

    fn router() -> Router {
        Router::new()
            .nest("/api/v1", routes::api_v1())
            .route("/health", get(|| async { "success" }))
            .fallback(|| async {
                Response::<String>::new(Some(404), Some("Not Found".to_string()), None)
            })
            .layer(
                ServiceBuilder::new()
                    .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
                    .layer(
                        TraceLayer::new_for_http()
                            .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                            .on_request(DefaultOnRequest::new().level(Level::INFO))
                            .on_response(
                                DefaultOnResponse::new()
                                    .level(Level::INFO)
                                    .include_headers(false),
                            ),
                    )
                    .layer(PropagateRequestIdLayer::x_request_id())
                    .layer(CorsLayer::permissive()),
            )
    }
}
