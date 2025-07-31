use actix_web::{
    HttpResponse, Responder, get,
    http::StatusCode,
    post,
    web::{Data, Json, Query},
};
use chrono::Utc;

use crate::{
    api::{create_payment::Payment, get_summary::SummaryQuery, state::AppState},
    core::{ApiError, Error, get_summary::get_summary},
    models::QueuedPayment,
};

#[post("/payments")]
async fn payments(body: Json<Payment>, state: Data<AppState>) -> Result<impl Responder, Error> {
    let now = Utc::now();

    let payment: QueuedPayment = QueuedPayment {
        correlation_id: String::from(&body.correlation_id),
        amount: body.amount,
        requested_at: now,
    };

    state.sender.send(payment).await?;

    Ok(HttpResponse::Ok().body("Pagamento criado 🫡"))
}

#[get("/payments-summary")]
async fn summary(
    query: Query<SummaryQuery>,
    state: Data<AppState>,
) -> Result<impl Responder, ApiError> {
    let conn = state.pool.get().await;

    if conn.is_err() {
        return Err(ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            "erro".to_string(),
        ));
    }

    get_summary(conn.unwrap(), query).await
}
