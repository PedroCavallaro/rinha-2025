use std::{
    sync::{
        Arc, LazyLock,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use reqwest::{Client, Method, Request, Url};

use crate::{config::CONFIG, models::HealtCheckResponse};

use super::{PaymentProcessor, error::Result};

pub static USE_FALLBACK: LazyLock<AtomicBool> = LazyLock::new(|| AtomicBool::new(false));

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
