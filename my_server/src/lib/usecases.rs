use crate::entities::Thing;
use crate::repos_interfaces::ThingRepo;

use anyhow::{Result, anyhow};

pub fn root() -> &'static str {
    "it works :)"
}

pub fn things_list<T: ThingRepo>(repo: T) -> Result<Vec<Thing>> {
    Err(anyhow!("TODO"))
}

pub fn add_thing<T: ThingRepo>(new_thing: Thing, repo: T) -> Result<Thing> {
    Err(anyhow!("TODO"))
}

pub fn get_thing<T: ThingRepo>(thing_id: u32, repo: T) -> Result<Thing> {
    Err(anyhow!("TODO"))
}

pub fn del_thing<T: ThingRepo>(thing_id: u32, repo: T) -> Result<Thing> {
    Err(anyhow!("TODO"))
}
