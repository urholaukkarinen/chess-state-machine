use crate::{
    color::Color,
    piece::{Piece, PieceType},
    rule::{MoveRule, MoveType, PieceMove},
    square::Square,
};

#[derive(Copy, Clone, Debug)]
pub struct Board {
    pieces: [[Option<Piece>; 8]; 8],
    active_color: Color,

    white_castling: Castling,
    black_castling: Castling,

    en_passant_target: Option<Square>,

    half_moves: u32,
    full_moves: u32,
}

impl Board {
    /// Initialize a new empty chess board
    pub fn empty() -> Self {
        Self {
            pieces: [[None; 8]; 8],
            active_color: Color::White,
            white_castling: Castling::both(),
            black_castling: Castling::both(),
            en_passant_target: None,
            half_moves: 0,
            full_moves: 1,
        }
    }

    pub fn set_piece(&mut self, x: u8, y: u8, piece_type: PieceType, color: Color) {
        self.pieces[x as usize][y as usize] = Some(Piece::new(piece_type, color, (x, y)))
    }

    pub fn piece(&self, x: u8, y: u8) -> &Option<Piece> {
        &self.pieces[x as usize][y as usize]
    }

    pub fn piece_mut(&mut self, x: u8, y: u8) -> &mut Option<Piece> {
        &mut self.pieces[x as usize][y as usize]
    }

    pub fn active_color(&self) -> Color {
        self.active_color
    }

    pub fn active_color_mut(&mut self) -> &mut Color {
        &mut self.active_color
    }

    pub fn white_castling(&self) -> Castling {
        self.white_castling
    }

    pub fn white_castling_mut(&mut self) -> &mut Castling {
        &mut self.white_castling
    }

    pub fn black_castling(&self) -> Castling {
        self.black_castling
    }

    pub fn black_castling_mut(&mut self) -> &mut Castling {
        &mut self.black_castling
    }

    pub fn en_passant_target(&self) -> Option<Square> {
        self.en_passant_target
    }

    pub fn en_passant_target_mut(&mut self) -> &mut Option<Square> {
        &mut self.en_passant_target
    }

    pub fn half_moves(&self) -> u32 {
        self.half_moves
    }

    pub fn half_moves_mut(&mut self) -> &mut u32 {
        &mut self.half_moves
    }

    pub fn full_moves(&self) -> u32 {
        self.full_moves
    }

    pub fn full_moves_mut(&mut self) -> &mut u32 {
        &mut self.full_moves
    }

    pub fn pieces(&self) -> Vec<Piece> {
        self.pieces
            .iter()
            .flatten()
            .filter_map(|p| *p)
            .collect::<Vec<_>>()
    }

    pub fn change_piece_type<S>(&mut self, square: S, new_piece_type: PieceType)
    where
        S: Into<Square>,
    {
        let Square { x, y } = square.into();

        if let Some(piece) = self.piece_mut(x, y).as_mut() {
            *piece.piece_type_mut() = new_piece_type;
        }
    }

    /// Play a move on the board.
    /// The result indicates whether the move was valid regular move or a pawn promotion.
    ///
    /// # Examples
    /// ```
    /// # use chess_state_machine::board::Board;
    /// # let mut board = Board::empty();
    /// let result = board.play_move((3, 0), (3, 1));
    /// ```
    pub fn play_move(&mut self, from: impl Into<Square>, to: impl Into<Square>) -> MoveResult {
        let from: Square = from.into();
        let to: Square = to.into();

        if let Some(mut piece) = *self.piece(from.x, from.y) {
            let valid_move = self
                .valid_moves(&piece, &from, true)
                .into_iter()
                .find(|valid_move| valid_move.target == to);

            if let Some(valid_move) = valid_move {
                if piece.piece_type() == PieceType::Pawn || self.piece(to.x, to.y).is_some() {
                    self.half_moves = 0;
                } else {
                    self.half_moves += 1;
                }

                piece.increment_move_count();

                if let Some(en_passant_target) = self.en_passant_target {
                    if valid_move.move_type == MoveType::PawnCapture
                        && valid_move.target == en_passant_target
                    {
                        let capture_y = if en_passant_target.y == 2 { 3 } else { 4 };
                        *self.piece_mut(to.x, capture_y) = None;
                    }
                }

                if valid_move.move_type == MoveType::Castling {
                    let (rook_from_x, rook_to_x) = if to.x > from.x {
                        (7, to.x - 1)
                    } else {
                        (0, to.x + 1)
                    };

                    let mut rook = self.piece(rook_from_x, to.y).unwrap();
                    rook.increment_move_count();

                    *self.piece_mut(rook_from_x, to.y) = None;
                    *self.piece_mut(rook_to_x, to.y) = Some(rook);
                }

                *self.piece_mut(from.x, from.y) = None;
                *self.piece_mut(to.x, to.y) = Some(piece);

                self.update_en_passant(&valid_move);
                self.update_castling_availability(&piece);

                self.active_color = if self.active_color == Color::Black {
                    self.full_moves += 1;
                    Color::White
                } else {
                    Color::Black
                };

                return if (to.y == 0 || to.y == 7) && piece.piece_type() == PieceType::Pawn {
                    MoveResult::PawnPromote
                } else {
                    MoveResult::Ok
                };
            }
        }

        MoveResult::Invalid
    }

    fn update_en_passant(&mut self, piece_move: &PieceMove) {
        let mut en_passant_target = None;

        if piece_move.move_type == MoveType::PawnDoubleMove {
            if piece_move.target.y == 3 {
                en_passant_target = Some((piece_move.target.x, 2).into());
            } else if piece_move.target.y == 4 {
                en_passant_target = Some((piece_move.target.x, 5).into());
            }
        }

        self.en_passant_target = en_passant_target;
    }

    fn update_castling_availability(&mut self, moved_piece: &Piece) {
        let castling = match moved_piece.color() {
            Color::Black => &mut self.black_castling,
            Color::White => &mut self.white_castling,
        };

        match moved_piece.piece_type() {
            PieceType::King => *castling = Castling::none(),
            PieceType::Rook if moved_piece.initial_square().x == 0 => castling.kingside = false,
            PieceType::Rook if moved_piece.initial_square().x == 7 => castling.queenside = false,
            _ => {}
        }
    }

    pub fn valid_moves(
        &self,
        piece: &Piece,
        square: &Square,
        check_king_safety: bool,
    ) -> Vec<PieceMove> {
        let mut valid_moves = Vec::new();

        for move_rule in piece.move_rules().iter() {
            valid_moves.extend(self.valid_moves_for_rule(
                piece,
                square,
                move_rule,
                check_king_safety,
            ));
        }

        valid_moves
    }

    fn valid_moves_for_rule(
        &self,
        piece: &Piece,
        square: &Square,
        move_rule: &MoveRule,
        check_king_safety: bool,
    ) -> Vec<PieceMove> {
        let mut valid_moves = Vec::new();

        let target_square = Square::from((
            (square.x as i8 + move_rule.x_offset) as _,
            (square.y as i8 + move_rule.y_offset) as _,
        ));

        if target_square.x > 7 || target_square.y > 7 {
            return valid_moves;
        }

        let target = *self.piece(target_square.x, target_square.y);
        let move_type = move_rule.move_type;

        match move_type {
            MoveType::Normal => {
                if target.filter(|p| p.color() == piece.color()).is_none() {
                    // Target square must be either empty or have different color piece

                    valid_moves.push(PieceMove {
                        move_type,
                        target: target_square,
                    });
                }
            }
            MoveType::LineOfSight => {
                if target.filter(|p| p.color() == piece.color()).is_none() {
                    // Target square must be either empty or have different color piece

                    valid_moves.push(PieceMove {
                        move_type,
                        target: target_square,
                    });

                    if target.is_none() {
                        valid_moves.extend(self.valid_moves_for_rule(
                            piece,
                            &target_square,
                            move_rule,
                            check_king_safety,
                        ))
                    }
                }
            }
            MoveType::PawnSingleMove => {
                if target.is_none() {
                    // Target square must be empty

                    valid_moves.push(PieceMove {
                        move_type,
                        target: target_square,
                    });
                }
            }
            MoveType::PawnDoubleMove => {
                if !piece.has_moved() && target.is_none() {
                    // Target square must be empty and must be first move

                    valid_moves.push(PieceMove {
                        move_type,
                        target: target_square,
                    });
                }
            }
            MoveType::PawnCapture => {
                if target.filter(|p| p.color() != piece.color()).is_some()
                    || Some(target_square).eq(&self.en_passant_target)
                {
                    // Target square must have different color piece
                    // or it must be an active en passant target.

                    valid_moves.push(PieceMove {
                        move_type,
                        target: target_square,
                    });
                }
            }
            MoveType::Castling => {
                if !piece.has_moved() && target.is_none() {
                    let (rook_x, dir_x) = if move_rule.x_offset < 0 {
                        (0, -1)
                    } else {
                        (7, 1)
                    };

                    let rook_has_not_moved = self
                        .piece(rook_x, target_square.y)
                        .filter(|p| p.piece_type() == PieceType::Rook && !p.has_moved())
                        .is_some();

                    if rook_has_not_moved && self
                            .valid_moves_for_rule(
                                piece,
                                square,
                                &MoveRule::line_of_sight(dir_x, 0),
                                check_king_safety,
                            )
                            .len()
                            == (square.x as i8 - rook_x as i8).abs() as usize - 1 && !self.is_king_threatened(piece.color()) {
                        valid_moves.push(PieceMove {
                            move_type,
                            target: target_square,
                        })
                    }
                }
            }
        }

        if check_king_safety {
            valid_moves.retain(|valid_move| {
                let mut board_copy = *self;
                *board_copy.piece_mut(valid_move.target.x, valid_move.target.y) = Some(*piece);
                *board_copy.piece_mut(square.x, square.y) = None;

                !board_copy.is_king_threatened(piece.color())
            });
        }

        valid_moves
    }

    fn is_king_threatened(&self, color: Color) -> bool {
        let (_, king_square) = self.find_piece(PieceType::King, color).unwrap();

        for x in 0..8 {
            for y in 0..8 {
                if let Some(piece) = self.piece(x, y).filter(|p| p.color() != color).as_ref() {
                    for piece_move in self.valid_moves(piece, &Square::from((x, y)), false) {
                        if piece_move.target == king_square {
                            return true;
                        }
                    }
                }
            }
        }

        false
    }

    fn find_piece(&self, piece_type: PieceType, piece_color: Color) -> Option<(Piece, Square)> {
        for x in 0..8 {
            for y in 0..8 {
                if let Some(piece) = self
                    .piece(x, y)
                    .filter(|p| p.piece_type() == piece_type && p.color() == piece_color)
                {
                    return Some((piece, Square::from((x, y))));
                }
            }
        }
        None
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Castling {
    pub kingside: bool,
    pub queenside: bool,
}

impl Castling {
    pub fn none() -> Self {
        Self {
            kingside: false,
            queenside: false,
        }
    }
    pub fn both() -> Self {
        Self {
            kingside: true,
            queenside: true,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MoveResult {
    Ok,
    PawnPromote,
    Invalid,
}
