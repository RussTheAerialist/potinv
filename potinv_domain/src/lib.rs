use minimal_id::MinimalId;
use std::path::Path;

#[derive(Debug, Default)]
struct Manufacturer;
#[derive(Debug, Default)]
struct ProductionNote;
#[derive(Debug, Default)]
struct Cost;

#[derive(Debug, Default)]
struct Clay {
    manufacturer: Manufacturer,
    item_number: String,
    cost_by_pound: Cost,
}

#[derive(Debug, Default)]
struct Glaze {
    manufacturer: Manufacturer,
    item_number: String,
    cost_by_liter: Cost,
}

#[derive(Debug, PartialEq)]
enum Stage {
    Planned,
}
impl Default for Stage {
    fn default() -> Stage {
        Stage::Planned
    }
}

#[derive(Debug, Default)]
struct Piece {
    id: MinimalId,
    description: String,
    clay: Option<Clay>,
    glazes: Vec<Glaze>,
    stage: Stage,
    photo_paths: Vec<Box<Path>>,
    production_notes: Vec<ProductionNote>,
}
impl Piece {
    fn new() -> Self {
        Self::default()
    }
}

impl PartialEq for Piece {
    fn eq(&self, rhs: &Piece) -> bool {
        self.id == rhs.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use minimal_id;

    #[test]
    fn defaults_are_correct() {
        let piece = Piece::new();
        assert_eq!(piece.stage, Stage::Planned);
        assert!(piece.clay.is_none());
        assert!(piece.glazes.is_empty());
        assert!(piece.photo_paths.is_empty());
        assert!(piece.production_notes.is_empty());
    }

    #[test]
    fn equality_is_based_on_entity_id() {
        let mut piece1 = Piece::new();
        let mut piece2 = Piece::new();
        let generator = minimal_id::Generator::default();
        let id = generator.generate();

        piece1.id = id;
        piece2.id = id;
        assert_eq!(piece1, piece2);

        piece1.id = generator.generate();
        assert_ne!(piece1, piece2);
    }
}
