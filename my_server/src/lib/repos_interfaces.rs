use crate::entities::Thing;

use anyhow::Result;
use async_trait::async_trait;


#[async_trait]
pub trait ThingRepo {
    async fn list(&mut self) -> Result<Vec<Thing>>;

    async fn del(&mut self, id: u32) -> Result<Option<Thing>>;

    async fn get(&mut self, id: u32) -> Result<Option<Thing>>;

    async fn add(&mut self, new_thing: Thing) -> Result<Thing>;
}