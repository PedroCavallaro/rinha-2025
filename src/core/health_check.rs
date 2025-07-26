use std::sync::Arc;

use reqwest::{Client, Method, Request, Url};

use crate::models::HealtCheckResponse;

use super::error::Result;

pub struct HealthChecker {
    client: Arc<Client>,
}

impl HealthChecker {
    pub fn new(client: Arc<Client>) -> Self {
        Self { client }
    }

    pub async fn check(&self, endpoint: &String) -> Result<HealtCheckResponse> {
        let url = format!("{}/payments/service-health", endpoint);

        let request = Request::new(Method::GET, Url::parse(url.as_str())?);

        let res = self
            .client
            .execute(request)
            .await?
            .json::<HealtCheckResponse>()
            .await?;

        Ok(res)
    }
}
