//! This are the pure domain objects
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Default)]
pub enum ThingType {
    #[default]
    One,
    Two,
    Three,
}

#[derive(Clone, Serialize, Deserialize, Default)]
pub struct Thing {
    pub id: u32,
    pub name: String,
    pub kind: ThingType,
    pub description: String,
}
