use std::{env, net::SocketAddr, sync::Arc};

use actix_web::{App, HttpServer, web};

use deadpool_redis::{Config, Runtime};
use dotenv::dotenv;

use reqwest::Client;
use rinha::{
    api::{
        routes::{payments, summary},
        state::AppState,
    },
    config::CONFIG,
    core::{health_check::HealthChecker, payment_processor::PaymentProcessor},
    models::QueuedPayment,
    queue::payment_consumer::PaymentsConsumer,
};
use tokio::sync::mpsc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let (sender, mut reciever) = mpsc::channel::<QueuedPayment>(20000);

    let config = Config::from_url(&CONFIG.redis_url);

    let corotine_pool = config.create_pool(Some(Runtime::Tokio1)).unwrap();

    let client = Arc::new(Client::new());

    HealthChecker::start(Arc::clone(&client));

    tokio::spawn(async move {
        let redis = corotine_pool.get().await.unwrap();
        let mut queue_processor = PaymentsConsumer::new(client, redis);

        while let Some(payment) = reciever.recv().await {
            let _ = queue_processor
                .handle_payment(PaymentProcessor::Default, payment)
                .await;
        }
    });

    let pool = config.create_pool(Some(Runtime::Tokio1)).unwrap();
    let app_state = AppState { pool, sender };

    HttpServer::new(move || {
        App::new()
            .service(payments)
            .service(summary)
            .app_data(web::Data::new(app_state.clone()))
    })
    .bind(&SocketAddr::from(([0, 0, 0, 0], 8080)))?
    .run()
    .await
}
