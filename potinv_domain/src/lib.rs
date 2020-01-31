#[macro_use] extern crate derive_builder;

pub mod piece;
pub mod events;

pub use piece::Piece;

#[derive(Debug, Default, Clone)]
pub struct Manufacturer;

#[derive(Debug, Default, Clone)]
pub struct ProductionNote;

#[derive(Debug, Default, Clone)]
pub struct Cost;

#[derive(Debug, Default, Clone)]
pub struct Clay {
    manufacturer: Manufacturer,
    item_number: String,
    cost_by_pound: Cost,
}

#[derive(Debug, Default, Clone)]
pub struct Glaze {
    manufacturer: Manufacturer,
    item_number: String,
    cost_by_liter: Cost,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Stage {
    Planned,
}

impl Default for Stage {
    fn default() -> Stage {
        Stage::Planned
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
