use serde::{Deserialize, Serialize};

use crate::models::{Model, ResourceCollection, ResourceIndex};

// ------------------------------------------------------------------------------------------------
// --- SimpleDataStorage
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct SimpleResourceStorage<M: Model<M>> {
    rows: ResourceCollection<M>,
    pk_index: ResourceIndex<M, M::K>,
}

#[allow(unused)]
impl<M: Model<M>> SimpleResourceStorage<M> {
    pub fn new(rows: ResourceCollection<M>) -> Self {
        let pk_index = M::create_pk_index(&rows);

        Self {
            rows,
            pk_index,
        }
    }

    pub fn rows(&self) -> &ResourceCollection<M> {
        &self.rows
    }

    pub fn pk_index(&self) -> &ResourceIndex<M, M::K> {
        &self.pk_index
    }
}
