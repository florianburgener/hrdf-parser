use core::fmt;

#[allow(unused)]
#[derive(Debug)]
pub struct Stop {
    pub id: i32,
    name: String,
    long_name: Option<String>,
    abbreviation: Option<String>,
    synonyms: Option<Vec<String>>,
    pub wgs_coordinate: Option<Coordinate>,
}

impl Stop {
    pub fn new(
        id: i32,
        name: String,
        long_name: Option<String>,
        abbreviation: Option<String>,
        synonyms: Option<Vec<String>>,
    ) -> Self {
        Self {
            id,
            name,
            long_name,
            abbreviation,
            synonyms,
            wgs_coordinate: None,
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct Coordinate {
    x: f64,
    y: f64,
    z: i16,
}

impl Coordinate {
    pub fn new(x: f64, y: f64, z: i16) -> Self {
        Self { x, y, z }
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.y, self.x)
    }
}
