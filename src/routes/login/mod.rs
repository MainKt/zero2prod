use actix_web::{http::header::ContentType, HttpResponse};

mod post;
pub use post::login;

pub async fn login_form() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("login.html"))
}
