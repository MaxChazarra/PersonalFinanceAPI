use serde::{Deserialize, Deserializer};
use uuid::Uuid;

pub fn deserialize_uuid<'de, D>(deserializer: D) -> Result<Uuid, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(Uuid::new_v4())
    } else {
        Uuid::parse_str(&s).map_err(serde::de::Error::custom)
    }
}