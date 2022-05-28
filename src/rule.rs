use crate::square::Square;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MoveType {
    Normal,
    LineOfSight,
    PawnSingleMove,
    PawnDoubleMove,
    PawnCapture,
    Castling,
}

pub struct MoveRule {
    pub move_type: MoveType,
    pub x_offset: i8,
    pub y_offset: i8,
}

impl MoveRule {
    pub fn normal(x: i8, y: i8) -> Self {
        Self::new(MoveType::Normal, x, y)
    }

    pub fn line_of_sight(x: i8, y: i8) -> Self {
        Self::new(MoveType::LineOfSight, x, y)
    }

    pub fn pawn_capture(x: i8, y: i8) -> Self {
        Self::new(MoveType::PawnCapture, x, y)
    }

    pub fn pawn_single_move(y: i8) -> Self {
        Self::new(MoveType::PawnSingleMove, 0, y)
    }

    pub fn pawn_double_move(y: i8) -> Self {
        Self::new(MoveType::PawnDoubleMove, 0, y)
    }

    pub fn castling(x: i8) -> Self {
        Self::new(MoveType::Castling, x, 0)
    }

    pub fn new(move_type: MoveType, x_offset: i8, y_offset: i8) -> Self {
        Self {
            move_type,
            x_offset,
            y_offset,
        }
    }
}

pub struct PieceMove {
    pub move_type: MoveType,
    pub target: Square,
}
