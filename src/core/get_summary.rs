use actix_web::{HttpResponse, Responder, http::StatusCode, web::Query};
use deadpool_redis::Connection;
use redis::AsyncCommands;

use crate::{
    api::get_summary::SummaryQuery,
    models::{PaymentProcessorsSummary, PaymentSummary},
};

use super::{ApiError, PaymentProcessor};

pub fn parse_from_to(query: Query<SummaryQuery>) -> (i64, i64) {
    let from = match query.from {
        Some(val) => val.timestamp(),
        None => 1,
    };

    let to = match query.to {
        Some(val) => val.timestamp(),
        None => i64::MAX,
    };

    (from, to)
}

pub async fn get_summary(
    mut conn: Connection,
    query: Query<SummaryQuery>,
) -> Result<impl Responder, ApiError> {
    let (from, to) = parse_from_to(query);

    let total = conn
        .zrangebyscore(
            PaymentProcessor::summary_key(&PaymentProcessor::Default),
            from,
            to,
        )
        .await;

    if total.is_err() {
        return Err(ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            "erro".to_string(),
        ));
    }

    let total: Vec<String> = total.unwrap();

    let total_fallabck = conn
        .zrangebyscore(
            PaymentProcessor::summary_key(&PaymentProcessor::Fallback),
            from,
            to,
        )
        .await;

    if total_fallabck.is_err() {
        return Err(ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            "erro".to_string(),
        ));
    }

    let total_fallabck: Vec<String> = total_fallabck.unwrap();

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
