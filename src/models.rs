use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueuedPayment {
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
    pub amount: f64,
    #[serde(rename = "requestedAt")]
    pub requested_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PaymentProcessorsSummary {
    default: PaymentSummary,
    fallback: PaymentSummary,
}

impl PaymentProcessorsSummary {
    pub fn new(default: PaymentSummary, fallback: PaymentSummary) -> Self {
        Self { default, fallback }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentSummary {
    #[serde(rename = "totalRequests")]
    pub total_requests: u32,
    #[serde(rename = "totalAmount")]
    pub total_amount: f64,
}
