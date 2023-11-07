#[derive(Debug)]
pub struct Stop {
    id: i32,
    name: String,
}

impl Stop {
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }
}
