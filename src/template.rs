use askama::Template;
use axum::{http::StatusCode, response::{Html, IntoResponse}};

#[derive(Template)]
#[template(path="index.html")]
pub struct IndexTemplate{}

use crate::todo::Todo;

#[derive(Template)]
#[template(path = "todos.html")]
pub struct TodosTemplate{
    // all fields passed in template can be used by jinja
    pub todos: Vec<Todo>,
}

pub struct HtmlTemplate<T>(pub T);

impl <T> IntoResponse for HtmlTemplate<T> where T: Template {
    fn into_response(self) -> axum::response::Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to render template. Error {}", err),
            ).into_response(),
        }
    }
}