// ------------------------------------------------------------------------------------------------
// --- AutoIncrement
// ------------------------------------------------------------------------------------------------

use std::cell::RefCell;

pub struct AutoIncrement {
    value: RefCell<i32>,
}

impl AutoIncrement {
    pub fn new() -> Self {
        Self {
            value: RefCell::new(0),
        }
    }

    pub fn next(&self) -> i32 {
        *self.value.borrow_mut() += 1;
        *self.value.borrow()
    }
}
