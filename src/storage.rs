use std::{collections::HashMap, rc::Rc};

use crate::models::{
    JourneyPlatformCollection, JourneyPlatformPrimaryIndex, Model, PlatformCollection,
    PlatformPrimaryIndex,
    TransportTypeCollection, TransportTypePrimaryIndex,
};

// ------------------------------------------------------------------------------------------------
// --- SimpleDataStorage
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct SimpleDataStorage<T> {
    rows: Vec<Rc<T>>,
    primary_index: HashMap<i32, Rc<T>>,
}

#[allow(unused)]
impl<T: Model<T>> SimpleDataStorage<T> {
    pub fn new(rows: Vec<Rc<T>>) -> Self {
        let primary_index = T::create_primary_index(&rows);

        Self {
            rows,
            primary_index,
        }
    }

    pub fn rows(&self) -> &Vec<Rc<T>> {
        &self.rows
    }

    pub fn primary_index(&self) -> &HashMap<i32, Rc<T>> {
        &self.primary_index
    }
}

// ------------------------------------------------------------------------------------------------
// --- JourneyPlatformData
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct JourneyPlatformData {
    rows: JourneyPlatformCollection,
    primary_index: JourneyPlatformPrimaryIndex,
}

#[allow(unused)]
impl JourneyPlatformData {
    pub fn new(
        rows: JourneyPlatformCollection,
        primary_index: JourneyPlatformPrimaryIndex,
    ) -> Self {
        Self {
            rows,
            primary_index,
        }
    }

    pub fn rows(&self) -> &JourneyPlatformCollection {
        &self.rows
    }

    pub fn primary_index(&self) -> &JourneyPlatformPrimaryIndex {
        &self.primary_index
    }
}

// ------------------------------------------------------------------------------------------------
// --- PlatformData
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct PlatformData {
    rows: PlatformCollection,
    primary_index: PlatformPrimaryIndex,
}

#[allow(unused)]
impl PlatformData {
    pub fn new(rows: PlatformCollection, primary_index: PlatformPrimaryIndex) -> Self {
        Self {
            rows,
            primary_index,
        }
    }

    pub fn rows(&self) -> &PlatformCollection {
        &self.rows
    }

    pub fn primary_index(&self) -> &PlatformPrimaryIndex {
        &self.primary_index
    }
}

// ------------------------------------------------------------------------------------------------
// --- TransportTypeData
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct TransportTypeData {
    rows: TransportTypeCollection,
    primary_index: TransportTypePrimaryIndex,
}

#[allow(unused)]
impl TransportTypeData {
    pub fn new(rows: TransportTypeCollection, primary_index: TransportTypePrimaryIndex) -> Self {
        Self {
            rows,
            primary_index,
        }
    }

    pub fn rows(&self) -> &TransportTypeCollection {
        &self.rows
    }

    pub fn primary_index(&self) -> &TransportTypePrimaryIndex {
        &self.primary_index
    }
}
