use serde::{Deserialize, Serialize};

use crate::models::{Model, ResourceCollection, ResourceIndex};

// ------------------------------------------------------------------------------------------------
// --- SimpleDataStorage
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleResourceStorage<M: Model<M>> {
    rows: ResourceCollection<M>,
    primary_index: ResourceIndex<M, M::K>,
}

#[allow(unused)]
impl<M: Model<M>> SimpleResourceStorage<M> {
    pub fn new(rows: ResourceCollection<M>) -> Self {
        let primary_index = M::create_primary_index(&rows);

        Self {
            rows,
            primary_index,
        }
    }

    pub fn rows(&self) -> &ResourceCollection<M> {
        &self.rows
    }

    pub fn primary_index(&self) -> &ResourceIndex<M, M::K> {
        &self.primary_index
    }
}
