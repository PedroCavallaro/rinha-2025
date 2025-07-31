use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct Payment {
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
    pub amount: f64,
}

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

#[derive(Default, Debug, Deserialize)]
pub struct HealtCheckResponse {
    pub failing: bool,
    #[serde(rename = "minResponseTime")]
    pub min_response_time: u16,
}

impl HealtCheckResponse {
    pub fn new(failing: bool, min_response_time: u16) -> Self {
        Self {
            failing,
            min_response_time,
        }
    }
}
