use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: Uuid,
    pub amount: f64,
    pub description: String,
}