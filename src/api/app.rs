use std::env;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

use axum::Router;
use dotenvy::dotenv;
use tower_http::cors::CorsLayer;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};

use crate::api::route;

pub struct App {
    pub listen: SocketAddr,
    pub router: Router,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        dotenv().ok();

        App {
            listen: App::listen(),
            router: App::router(),
        }
    }

    fn listen() -> SocketAddr {
        let listen = env::var("APP_LISTEN").unwrap_or_else(|_| String::from("0.0.0.0"));
        let port = env::var("APP_PORT").map_or_else(|_| 8080, |v| v.parse().unwrap());

        SocketAddr::from((IpAddr::from_str(listen.as_str()).unwrap(), port))
    }

    fn router() -> Router {
        Router::new()
            .nest("/health", route::health())
            .nest("/api/v1", route::api_v1())
            .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
            .layer(PropagateRequestIdLayer::x_request_id())
            .layer(CorsLayer::permissive())
    }
}
