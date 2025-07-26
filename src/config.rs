use std::{env, sync::LazyLock};

use dotenv::dotenv;

use crate::core::PaymentProcessor;

pub struct Config {
    pub redis_url: String,
    pub payment_processor_default: String,
    pub payment_processor_fallback: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        Self {
            redis_url: env::var("REDIS_URL").unwrap(),
            payment_processor_default: env::var("PAYMENT_PROCESSOR_API")
                .unwrap_or(String::from("http://localhost:8001")),
            payment_processor_fallback: env::var("PAYMENT_PROCESSOR_API_FALLBACK")
                .unwrap_or(String::from("http://localhost:8002")),
        }
    }

    pub fn get_proccessor_url(&self, payment_processor: &PaymentProcessor) -> &String {
        match payment_processor {
            PaymentProcessor::Default => &self.payment_processor_default,
            PaymentProcessor::Fallback => &self.payment_processor_fallback,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

pub static CONFIG: LazyLock<Config> = LazyLock::new(Config::new);
