use crate::{color::Color, rule::MoveRule, square::Square};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PieceType {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    piece_type: PieceType,
    color: Color,

    move_count: u32,
    last_move_turn: Option<u32>,

    initial_square: Square,
}

impl Piece {
    pub fn new<S: Into<Square>>(piece_type: PieceType, color: Color, initial_square: S) -> Self {
        let initial_square = initial_square.into();

        Self {
            piece_type,
            color,
            move_count: 0,
            last_move_turn: None,

            initial_square,
        }
    }

    pub fn piece_type(&self) -> PieceType {
        self.piece_type
    }

    pub fn piece_type_mut(&mut self) -> &mut PieceType {
        &mut self.piece_type
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn has_moved(&self) -> bool {
        self.move_count > 0
    }

    pub fn move_count(&self) -> u32 {
        self.move_count
    }

    pub fn increment_move_count(&mut self) {
        self.move_count += 1;
    }

    pub fn last_move_turn(&self) -> Option<u32> {
        self.last_move_turn
    }

    pub fn initial_square(&self) -> Square {
        self.initial_square
    }

    pub fn move_rules(&self) -> Vec<MoveRule> {
        match (self.piece_type, self.color) {
            (PieceType::Pawn, Color::Black) => vec![
                MoveRule::pawn_single_move(-1),
                MoveRule::pawn_double_move(-2),
                MoveRule::pawn_capture(-1, -1),
                MoveRule::pawn_capture(1, -1),
            ],
            (PieceType::Pawn, Color::White) => vec![
                MoveRule::pawn_single_move(1),
                MoveRule::pawn_double_move(2),
                MoveRule::pawn_capture(-1, 1),
                MoveRule::pawn_capture(1, 1),
            ],
            (PieceType::Rook, _) => vec![
                MoveRule::line_of_sight(1, 0),
                MoveRule::line_of_sight(-1, 0),
                MoveRule::line_of_sight(0, 1),
                MoveRule::line_of_sight(0, -1),
            ],
            (PieceType::Knight, _) => vec![
                MoveRule::normal(1, 2),
                MoveRule::normal(1, -2),
                MoveRule::normal(-1, 2),
                MoveRule::normal(-1, -2),
                MoveRule::normal(2, -1),
                MoveRule::normal(2, 1),
                MoveRule::normal(-2, -1),
                MoveRule::normal(-2, 1),
            ],
            (PieceType::Bishop, _) => vec![
                MoveRule::line_of_sight(1, 1),
                MoveRule::line_of_sight(-1, 1),
                MoveRule::line_of_sight(1, -1),
                MoveRule::line_of_sight(-1, -1),
            ],
            (PieceType::Queen, _) => vec![
                MoveRule::line_of_sight(1, 0),
                MoveRule::line_of_sight(-1, 0),
                MoveRule::line_of_sight(0, 1),
                MoveRule::line_of_sight(0, -1),
                MoveRule::line_of_sight(1, 1),
                MoveRule::line_of_sight(-1, 1),
                MoveRule::line_of_sight(1, -1),
                MoveRule::line_of_sight(-1, -1),
            ],
            (PieceType::King, _) => vec![
                MoveRule::normal(1, 1),
                MoveRule::normal(-1, -1),
                MoveRule::normal(1, -1),
                MoveRule::normal(-1, 1),
                MoveRule::normal(0, 1),
                MoveRule::normal(0, -1),
                MoveRule::normal(1, 0),
                MoveRule::normal(-1, 0),
                MoveRule::castling(-2),
                MoveRule::castling(-3),
                MoveRule::castling(2),
            ],
        }
    }
}
