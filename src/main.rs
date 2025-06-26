
mod todo;
mod template;

use axum::{response::IntoResponse, routing::get};
use tokio::net::TcpListener;

use crate::template::{HtmlTemplate, IndexTemplate};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = axum::Router::new()
    .route("/", get(index));
    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await?;
    Ok(())
}

async fn index() -> impl IntoResponse {
    // return HtmlTemplate to make it a valid responese
    HtmlTemplate(IndexTemplate {}) 
}