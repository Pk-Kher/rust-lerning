use std::fmt;
//external traits and local type
pub struct Pirate {
    pub name: String,
    bounty: u64,
}

impl Pirate {
    pub fn new(name: String, bounty: u64) -> Pirate {
        Pirate { name, bounty }
    }
}

impl fmt::Display for Pirate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} - bounty: {} berries", self.name, self.bounty)
    }
}

// local trais and external Type

pub trait PrirateHi {
    fn say_hi(&self) -> String;
}
impl PrirateHi for String {
    fn say_hi(&self) -> String {
        format!("hi, {}", self)
    }
}
