use super::*;
use std::path::Path;
use minimal_id::MinimalId;

pub type PieceId = MinimalId;

#[derive(Debug, Default, Builder)]
#[builder(setter(into, strip_option))]
pub struct Piece {
    pub id: PieceId,

    #[builder(default)]
    pub description: Option<String>,
    #[builder(default)]
    pub clay: Option<Clay>,
    #[builder(default)]
    pub glazes: Vec<Glaze>,
    #[builder(default)]
    pub stage: Stage,
    #[builder(default)]
    pub photo_paths: Vec<Box<Path>>,
    #[builder(default)]
    pub production_notes: Vec<ProductionNote>,
}

impl Piece {
    pub fn builder() -> PieceBuilder {
        PieceBuilder::default()
    }
}

impl PartialEq<Piece> for Piece {
    fn eq(&self, rhs: &Piece) -> bool {
        self.id == rhs.id
    }
}
impl PartialEq<PieceId> for Piece {
    fn eq(&self, rhs: &PieceId) -> bool {
        self.id == *rhs
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use minimal_id;

    fn piece_and_id() -> (Piece, PieceId) {
        let generator = minimal_id::Generator::default();
        let id = generator.generate();

        let piece = Piece::builder()
            .id(id)
            .build().unwrap();

        (piece, id)
    }

    #[test]
    fn defaults_are_correct() {
        let piece = Piece::default();
        assert_eq!(piece.stage, Stage::Planned);
        assert!(piece.clay.is_none());
        assert!(piece.glazes.is_empty());
        assert!(piece.photo_paths.is_empty());
        assert!(piece.production_notes.is_empty());
    }

    #[test]
    fn equality_is_based_on_entity_id() {
        let mut piece1 = Piece::default();
        let mut piece2 = Piece::default();
        let generator = minimal_id::Generator::default();
        let id = generator.generate();

        piece1.id = id;
        piece2.id = id;
        assert_eq!(piece1, piece2);

        piece1.id = generator.generate();
        assert_ne!(piece1, piece2);
    }

    #[test]
    fn can_build() {
        let (piece, id) = piece_and_id();
        assert_eq!(piece.id, id);
    }

    #[test]
    fn can_test_equality_with_id() {
        let (piece, id) = piece_and_id();
        assert_eq!(piece, id);
    }
}
