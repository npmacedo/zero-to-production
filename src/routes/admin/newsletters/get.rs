use actix_web::{http::header::ContentType, HttpResponse};
use actix_web_flash_messages::IncomingFlashMessages;
use std::fmt::Write;

pub async fn newsletter_form(
    flash_messages: IncomingFlashMessages,
) -> Result<HttpResponse, actix_web::Error> {
    let mut error_html = String::new();
    let idempotency_key = uuid::Uuid::new_v4().to_string();

    for m in flash_messages.iter() {
        writeln!(error_html, "<p><i>{}</i></p>", m.content()).unwrap();
    }

    let body = include_str!("newsletter.html")
        .replace("{msg_html}", &error_html)
        .replace("{idempotency_key}", &idempotency_key);

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(body))
}
