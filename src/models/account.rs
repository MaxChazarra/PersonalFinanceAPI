use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::utils::uuid::deserialize_uuid;
use super::transaction::Transaction;

#[derive(Serialize, Deserialize, Clone)]
pub struct Account {
    #[serde(deserialize_with = "deserialize_uuid")]
    pub id: Uuid,
    pub name: String,
    pub transactions: Vec<Transaction>,
}