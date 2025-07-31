use core::fmt;

pub enum PaymentProcessor {
    Default,
    Fallback,
}

impl fmt::Display for PaymentProcessor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PaymentProcessor::Default => write!(f, "Default"),
            PaymentProcessor::Fallback => write!(f, "Fallback"),
        }
    }
}

impl PaymentProcessor {
    pub fn summary_key(payment_processor: &PaymentProcessor) -> String {
        let key = format!("summary:{payment_processor}");

        key
    }
}
