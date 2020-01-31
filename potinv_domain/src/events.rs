use crate::Piece;

pub enum PieceEvent {
    PieceCreated(Piece)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Piece;
    use minimal_id::MinimalId;

    #[test]
    fn events_exist_test() {
        let event = PieceEvent::PieceCreated(Piece::default());
        match event {
            PieceEvent::PieceCreated(p) => assert_eq!(p.id, MinimalId::default()),
        }
    }
}
