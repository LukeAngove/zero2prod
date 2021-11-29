use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, db: web::Data<PgPool>) -> HttpResponse {
    log::info!(
        "Saving '{}' '{}' details to database as a new subscriber",
        form.email,
        form.name
    );
    let res = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(db.as_ref())
    .await;

    match res {
        Ok(_) => {
            log::info!("Saved subscriber details to database");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::info!("Failed to execute query {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
