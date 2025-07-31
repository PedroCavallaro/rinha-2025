use actix_web::{
    HttpResponse, Responder, get, post,
    web::{Data, Json, Query},
};
use chrono::{DateTime, Utc};
use redis::AsyncTypedCommands;
use serde::Deserialize;

use crate::{
    api::state::AppState,
    config::CONFIG,
    core::{PaymentProcessor, error::Error},
    models::{Payment, PaymentProcessorsSummary, PaymentSummary, QueuedPayment},
};

#[derive(Debug, Deserialize)]
struct SummaryQuery {
    from: Option<DateTime<Utc>>,
    to: Option<DateTime<Utc>>,
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
    let mut conn = state.pool.get().await?;

    let from = match query.from {
        Some(val) => val.timestamp(),
        None => 1,
    };

    let to = match query.to {
        Some(val) => val.timestamp(),
        None => i64::MAX,
    };

    let total = conn
        .zrangebyscore(
            PaymentProcessor::summary_key(&PaymentProcessor::Default),
            from,
            to,
        )
        .await?;

    let total_fallabck = conn
        .zrangebyscore(
            PaymentProcessor::summary_key(&PaymentProcessor::Fallback),
            from,
            to,
        )
        .await?;

    let total_amount_default: f64 = total.iter().map(|e| e.parse::<f64>().unwrap_or(0.0)).sum();

    let total_amount_fallback: f64 = total_fallabck
        .iter()
        .map(|e| e.parse::<f64>().unwrap_or(0.0))
        .sum();

    let summary = PaymentProcessorsSummary::new(
        PaymentSummary {
            total_amount: total_amount_default,
            total_requests: total.len() as u32,
        },
        PaymentSummary {
            total_requests: total_fallabck.len() as u32,
            total_amount: total_amount_fallback,
        },
    );

    Ok(HttpResponse::Ok().json(summary))
}
