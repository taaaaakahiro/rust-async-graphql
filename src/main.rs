#[macro_use]
extern crate thiserror;

mod config;
mod resolver;

use crate::config::Config;
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Request, Response, Schema,
};
use axum::{
    extract::Extension,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use resolver::QueryRoot;
use tokio::net::TcpListener;
use tracing_subscriber::fmt;

pub type BlogSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

#[tokio::main]
async fn main() {
    let event_format = fmt::format::json();
    tracing_subscriber::fmt().event_format(event_format).init();

    let cfg = Config::new();

    let server = async {
        let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription).finish();

        let app = Router::new()
            .route("/", get(graphql_playground).post(graphql_handler))
            .fallback(notfound_handler)
            .layer(Extension(schema));

        let addr = format!("0.0.0.0:{}", cfg.db.port);
        tracing::info!("listening on {}", addr);
        let addr = TcpListener::bind(addr).await.unwrap();
        axum::serve(addr, app).await.unwrap();
    };

    tokio::join!(server);
}

async fn graphql_handler(schema: Extension<BlogSchema>, req: Json<Request>) -> Json<Response> {
    schema.execute(req.0).await.into()
}

async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

async fn notfound_handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "not found")
}
