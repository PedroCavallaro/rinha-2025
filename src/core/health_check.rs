use std::{
    sync::{
        Arc, LazyLock,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use reqwest::Client;
use serde::Deserialize;

use crate::config::CONFIG;

use super::PaymentProcessor;

pub static USE_FALLBACK: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));

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

pub struct HealthChecker {}

impl HealthChecker {
    pub fn start(client: Arc<Client>) {
        tokio::spawn(async move {
            loop {
                let _ = HealthChecker::check(client.clone()).await;

                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        });
    }

    pub async fn check(client: Arc<Client>) {
        let url = format!(
            "{}/payments/service-health",
            CONFIG.get_proccessor_url(&PaymentProcessor::Default)
        );

        let res = client.get(url).send().await;

        match res {
            Ok(val) => {
                let json = val.json::<HealtCheckResponse>().await;

                if let Ok(_json) = json {
                    USE_FALLBACK.store(_json.failing, Ordering::Relaxed);

                    return;
                }

                USE_FALLBACK.store(true, Ordering::Relaxed);
            }
            Err(_) => {
                USE_FALLBACK.store(true, Ordering::Relaxed);
            }
        }
    }
}
