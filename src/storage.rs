use std::{collections::HashMap, rc::Rc};

use crate::models::Model;

// ------------------------------------------------------------------------------------------------
// --- SimpleDataStorage
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct SimpleDataStorage<T: Model<T>> {
    rows: Vec<Rc<T>>,
    primary_index: HashMap<T::U, Rc<T>>,
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

    pub fn primary_index(&self) -> &HashMap<T::U, Rc<T>> {
        &self.primary_index
    }
}
