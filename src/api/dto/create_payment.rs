use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Payment {
    #[serde(rename = "correlationId")]
    pub correlation_id: String,
    pub amount: f64,
}
