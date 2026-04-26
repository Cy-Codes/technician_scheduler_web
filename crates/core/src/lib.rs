use serde::{Deserialize, Serialize};

// Shared data models go here. Both frontend and backend depend on this crate.
// Types that cross the network boundary must derive Serialize + Deserialize.

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Technician {
    pub id: u32,
    pub name: String,
}
