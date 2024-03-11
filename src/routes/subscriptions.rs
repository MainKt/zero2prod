use actix_web::{web, HttpResponse, Responder};
use sqlx::types::chrono::Utc;
use sqlx::types::Uuid;
use sqlx::PgPool;

use unicode_segmentation::UnicodeSegmentation;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    )
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> impl Responder {
    if !is_valid_name(&form.name) {
        return HttpResponse::BadRequest().finish();
    }

    match insert_subscriber(&pool, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub fn is_valid_name(name: &str) -> bool {
    let is_empty_or_whitespace = name.trim().is_empty();

    let is_too_long = name.graphemes(true).count() > 256;

    let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
    let contains_forbidden_characters = name.chars().any(|g| forbidden_characters.contains(&g));

    !(is_empty_or_whitespace || is_too_long || contains_forbidden_characters)
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {e:?}");
        e
    })?;
    Ok(())
}
