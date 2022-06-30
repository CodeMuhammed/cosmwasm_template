use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
    Cancelled,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Priority {
    None,
    Low,
    Medium,
    High,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Entry {
    pub id: u64,
    pub description: String,
    pub status: Status,
    pub priority: Priority,
}

// This stores the config variables during initialization of the contract
pub const INIT_CONFIG: Item<Config> = Item::new("INIT_CONFIG");

// This keeps track of the number of items in the todo list
pub const ENTRY_SEQ: Item<u64> = Item::new("ENTRY_SEQ");

// This keeps track of a mapping between the todo id (ENTRY_SEQ) : Entry
pub const LIST: Map<u64, Entry> = Map::new("LIST");

// Limits for the custom range query
pub const MAX_LIMIT: u32 = 30;
pub const DEFAULT_LIMIT: u32 = 10;
