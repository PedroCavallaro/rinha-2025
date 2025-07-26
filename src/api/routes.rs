use actix_web::{
    HttpResponse, Responder, get, post,
    web::{Data, Json, Query},
};
use chrono::Utc;
use serde::Deserialize;

use crate::{
    api::state::AppState,
    core::error::Error,
    models::{Payment, QueuedPayment},
};

#[derive(Debug, Deserialize)]
struct SummaryQuery {
    from: String,
    to: String,
}

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
) -> Result<impl Responder, Error> {
    Ok("")
}
