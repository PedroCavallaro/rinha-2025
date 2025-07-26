use std::sync::Arc;

use deadpool_redis::Connection;
use redis::pipe;
use reqwest::{Client, Method};

use crate::{
    config::CONFIG, core::{ HealthChecker, PaymentProcessor, Result}, models::QueuedPayment
};

pub struct PaymentsConsumer {
    health_checker: HealthChecker,
    client: Arc<Client>,
    redis: Connection,
}

impl PaymentsConsumer {
    pub fn new(health_checker: HealthChecker, client: Arc<Client>, redis: Connection) -> Self {
        Self {
            health_checker,
            client,
            redis,
        }
    }

    async fn update_summary(&mut self, payment: QueuedPayment) -> Result<()> {
        let key = PaymentProcessor::summary_key(&PaymentProcessor::Default);
        let timestamp = payment.requested_at.timestamp();

        let json = serde_json::to_string(&payment)?;

        let _: () = pipe()
            .atomic()
            .zadd(key, json, timestamp)
            .query_async(&mut self.redis)
            .await?;

        Ok(())
    }

    pub async fn handle_payment(
        &mut self,
        current_processor: PaymentProcessor,
        payment: QueuedPayment,
    ) -> Result<()> {
        let url = format!("{}/payments", CONFIG.get_proccessor_url(&current_processor));

        let request = self
            .client
            .request(Method::POST, url)
            .json(&payment)
            .build()?;

        let res = self.client.execute(request).await?;

        if res.status() != 200 {
            return Ok(());
        }

        self.update_summary(payment).await?;

        Ok(())
    }
}
