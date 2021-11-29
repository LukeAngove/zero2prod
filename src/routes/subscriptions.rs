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
    let request_id = Uuid::new_v4();
    log::info!(
        "request_id {}: Saving '{}' '{}' details to database as a new subscriber",
        request_id,
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
            log::info!(
                "request_id {}: Saved subscriber details to database",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            log::info!("request_id {}: Failed to execute query {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
