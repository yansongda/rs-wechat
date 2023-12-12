use std::fmt::Debug;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use std::time::Duration;

use axum::http::Request;
use axum::routing::get;
use axum::{http, Router};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::request_id::{
    MakeRequestUuid, PropagateRequestIdLayer, RequestId, SetRequestIdLayer,
};
use tower_http::trace::{MakeSpan, OnRequest, OnResponse, TraceLayer};
use tracing::metadata::LevelFilter;
use tracing::{info, info_span, Level, Span};
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
                            .make_span_with(RequestIdMakeSpan)
                            .on_request(OnRequestLayer)
                            .on_response(OnResponseLayer),
                    )
                    .layer(PropagateRequestIdLayer::x_request_id())
                    .layer(CorsLayer::permissive()),
            )
    }
}

#[derive(Debug, Clone)]
struct RequestIdMakeSpan;

impl<B> MakeSpan<B> for RequestIdMakeSpan {
    fn make_span(&mut self, request: &Request<B>) -> Span {
        let request_id = request
            .extensions()
            .get::<RequestId>()
            .map(|request_id| request_id.header_value().to_str().unwrap())
            .unwrap_or_else(|| "unknown");

        info_span!("", request_id)
    }
}

#[derive(Debug, Clone)]
struct OnRequestLayer;

impl<B: Debug> OnRequest<B> for OnRequestLayer {
    fn on_request(&mut self, request: &Request<B>, _: &Span) {
        info!(
            method = %request.method(),
            uri = %request.uri(),
            headers = ?request.headers(),
            inputs = ?request.body(),
            "--> 接收到请求",
        );
    }
}

#[derive(Debug, Clone)]
struct OnResponseLayer;

impl<B: Debug> OnResponse<B> for OnResponseLayer {
    fn on_response(self, _: &http::Response<B>, latency: Duration, _: &Span) {
        info!(?latency, "<-- 请求处理完成");
    }
}
