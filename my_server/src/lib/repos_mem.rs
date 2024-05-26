use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::Mutex;
use once_cell::sync::Lazy;

use crate::repos_interfaces::ThingRepo;
use crate::entities::Thing;

use anyhow::Result;
use async_trait::async_trait;


static storage: Lazy<Arc<Mutex<HashMap<u32, Thing>>>> = Lazy::new(|| {
    Arc::new(Mutex::new(HashMap::new()))
});

pub struct InMemoryThingRepo {
}

impl InMemoryThingRepo {
    pub fn new() -> Self {
        InMemoryThingRepo {}
    }
}

#[async_trait]
impl ThingRepo for InMemoryThingRepo {
    async fn list(&mut self) -> Result<Vec<Thing>> {
        let mut l = storage.lock().await;
        let hash_map = &mut *l;
        let content: Vec<Thing> = hash_map.values().cloned().collect();
        Ok(content)
    }

    async fn del(&mut self, id: u32) -> Result<Option<Thing>> {
        let mut l = storage.lock().await;
        let hash_map = &mut *l;
        if let Some(v) = hash_map.remove(&id) {
            Ok(Some(v))
        } else {
            Ok(None)
        }
    }

    async fn get(&mut self, id: u32) -> Result<Option<Thing>> {
        let mut l = storage.lock().await;
        let hash_map = &mut *l;
        if let Some((_,v)) = hash_map.iter().find(|(k, _)| **k == id ) {
            Ok(Some(v.clone()))
        } else {
            Ok(None)
        }
    }

    async fn add(&mut self, new_thing: Thing) -> Result<Thing> {
        let mut l = storage.lock().await;
        let hash_map = &mut *l;
        hash_map.insert(new_thing.id, new_thing.clone());
        Ok(new_thing)
    }
}