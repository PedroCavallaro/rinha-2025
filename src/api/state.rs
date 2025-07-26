use deadpool_redis::Pool;
use tokio::sync::mpsc::Sender;

use crate::models::QueuedPayment;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool,
    pub sender: Sender<QueuedPayment>,
}
