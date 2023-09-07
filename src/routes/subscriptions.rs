use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing_futures::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

#[tracing::instrument(name = "Saving new subscriber details in the database", skip(db, form))]
async fn insert_subscriber(db: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
    let query_span = tracing::info_span!("Saving new subscriber details in the database (query)");

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
    .execute(db)
    .instrument(query_span)
    .await
    .map(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}

#[tracing::instrument(
        name = "Adding a new subscriber",
        skip(form, db),
        fields(
            subscriber_email = %form.email,
            subscriber_name = %form.name,
        )
)]
pub async fn subscribe(form: web::Form<FormData>, db: web::Data<PgPool>) -> HttpResponse {
    match insert_subscriber(&db, &form).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
