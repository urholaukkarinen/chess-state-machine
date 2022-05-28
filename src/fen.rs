use std::convert::TryFrom;

use crate::{
    board::{Board, Castling},
    color::Color,
    piece::{Piece, PieceType},
    square::Square,
};

pub trait FromFen {
    /// Constructs a new Self from given FEN string.
    ///
    /// https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
    fn from_fen(fen: &str) -> Self;
}

pub trait IntoFen {
    /// Constructs a new FEN string from Self.
    ///
    /// https://en.wikipedia.org/wiki/Forsyth%E2%80%93Edwards_Notation
    fn into_fen(self) -> String;
}

impl FromFen for Board {
    fn from_fen(fen: &str) -> Self {
        let mut board = Board::empty();

        let (piece_placement, fen) = fen.split_once(' ').unwrap();

        let mut file: i8 = 0;
        let mut rank: i8 = 7;

        for c in piece_placement.chars() {
            if c == '/' {
                // Jump to the next rank

                rank -= 1;
                file = 0;
                continue;
            }

            if let Some(empty_squares) = c.to_digit(10) {
                // Advance file by N empty squares

                file += empty_squares as i8;
            } else {
                let (piece_type, color) = fen_char_to_piece(c).unwrap();

                board.set_piece(file as _, rank as _, piece_type, color);
                file += 1;
                //
            }
        }

        let fen = fen.split_whitespace().collect::<Vec<_>>();

        *board.active_color_mut() = match fen[0] {
            "b" => Color::Black,
            "w" => Color::White,
            c => panic!("Invalid color: {}", c),
        };

        *board.white_castling_mut() = Castling::none();
        *board.black_castling_mut() = Castling::none();

        for c in fen[1].chars() {
            match c {
                'K' => board.white_castling_mut().kingside = true,
                'Q' => board.white_castling_mut().queenside = true,
                'k' => board.black_castling_mut().kingside = true,
                'q' => board.black_castling_mut().queenside = true,
                _ => {}
            }
        }

        if let Ok(en_passant_target) = Square::try_from(fen[2]) {
            *board.en_passant_target_mut() = Some(en_passant_target);
        }

        *board.half_moves_mut() = fen[3].parse::<u32>().unwrap();
        *board.full_moves_mut() = fen[4].parse::<u32>().unwrap();

        board
    }
}

impl IntoFen for Board {
    fn into_fen(self) -> String {
        let mut fen = String::new();

        for y in (0..8).rev() {
            let mut empty_squares = 0;

            for x in 0..8 {
                match self.piece(x, y).as_ref() {
                    Some(piece) => {
                        if empty_squares > 0 {
                            fen.push_str(&empty_squares.to_string());
                            empty_squares = 0;
                        }
                        fen.push_str(&piece_to_fen_char(piece));
                    }
                    None => empty_squares += 1,
                }
            }

            if empty_squares > 0 {
                fen.push_str(&empty_squares.to_string());
            }

            if y > 0 {
                fen.push('/');
            }
        }

        fen.push_str(match self.active_color() {
            Color::Black => " b",
            Color::White => " w",
        });

        let mut castling = String::new();
        if self.white_castling().kingside {
            castling.push('K');
        }
        if self.white_castling().queenside {
            castling.push('Q');
        }
        if self.black_castling().kingside {
            castling.push('k');
        }
        if self.black_castling().queenside {
            castling.push('q');
        }
        if castling.is_empty() {
            castling.push('-');
        }

        fen.push_str(&format!(" {}", castling));

        if let Some(target) = self.en_passant_target() {
            fen.push_str(&format!(" {}", target))
        } else {
            fen.push_str(" -")
        }

        fen.push_str(&format!(" {} {}", self.half_moves(), self.full_moves()));

        fen
    }
}

fn fen_char_to_piece(c: char) -> Option<(PieceType, Color)> {
    let piece_type = match c.to_lowercase().to_string().as_str() {
        "p" => PieceType::Pawn,
        "n" => PieceType::Knight,
        "b" => PieceType::Bishop,
        "r" => PieceType::Rook,
        "q" => PieceType::Queen,
        "k" => PieceType::King,
        _ => return None,
    };

    let color = match c.is_uppercase() {
        true => Color::White,
        false => Color::Black,
    };

    Some((piece_type, color))
}

fn piece_to_fen_char(piece: &Piece) -> String {
    let c = match piece.piece_type() {
        PieceType::Pawn => "p",
        PieceType::Knight => "n",
        PieceType::Bishop => "b",
        PieceType::Rook => "r",
        PieceType::Queen => "q",
        PieceType::King => "k",
    };

    match piece.color() {
        Color::Black => c.to_string(),
        Color::White => c.to_uppercase(),
    }
}

#[cfg(test)]
mod tests {
    use crate::board::MoveResult;

    use super::*;

    #[test]
    fn test_board_into_fen() {
        let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

        assert_eq!(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
            board.into_fen()
        );

        assert_eq!(
            MoveResult::Ok,
            board.play_move(
                Square::try_from("e2").unwrap(),
                Square::try_from("e4").unwrap(),
            )
        );

        assert_eq!(
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
            board.into_fen()
        );

        assert_eq!(
            MoveResult::Ok,
            board.play_move(
                Square::try_from("c7").unwrap(),
                Square::try_from("c5").unwrap(),
            )
        );

        assert_eq!(
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2",
            board.into_fen()
        );

        assert_eq!(
            MoveResult::Ok,
            board.play_move(
                Square::try_from("g1").unwrap(),
                Square::try_from("f3").unwrap(),
            )
        );

        assert_eq!(
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2",
            board.into_fen()
        );

        // Castling

        board = Board::from_fen("rnbqkbnr/5ppp/8/ppppp3/2PP4/N2QB3/PP2PPPP/R3KBNR w KQkq e6 0 6");

        assert_eq!(
            MoveResult::Ok,
            board.play_move(
                Square::try_from("e1").unwrap(),
                Square::try_from("c1").unwrap(),
            )
        );

        assert_eq!(
            "rnbqkbnr/5ppp/8/ppppp3/2PP4/N2QB3/PP2PPPP/2KR1BNR b kq - 1 6",
            board.into_fen()
        );

        // Black en-passant

        board = Board::from_fen("rnbqkbnr/5ppp/8/2ppp2P/ppPP4/N2QB3/PP2PPP1/2KR1BNR b kq - 0 8");

        assert_eq!(
            MoveResult::Ok,
            board.play_move(
                Square::try_from("g7").unwrap(),
                Square::try_from("g5").unwrap(),
            )
        );

        assert_eq!(
            "rnbqkbnr/5p1p/8/2ppp1pP/ppPP4/N2QB3/PP2PPP1/2KR1BNR w kq g6 0 9",
            board.into_fen()
        );

        assert_eq!(
            MoveResult::Ok,
            board.play_move(
                Square::try_from("h5").unwrap(),
                Square::try_from("g6").unwrap(),
            )
        );

        assert_eq!(
            "rnbqkbnr/5p1p/6P1/2ppp3/ppPP4/N2QB3/PP2PPP1/2KR1BNR b kq - 0 9",
            board.into_fen()
        );

        // White en-passant
        board =
            Board::from_fen("rn1q1b1r/1Q1bk2p/3N1nP1/2pp1p2/pPPPp3/4B3/P3PPP1/2KR1BNR b - b3 0 15");

        assert_eq!(
            MoveResult::Ok,
            board.play_move(
                Square::try_from("a4").unwrap(),
                Square::try_from("b3").unwrap(),
            )
        );

        assert_eq!(
            "rn1q1b1r/1Q1bk2p/3N1nP1/2pp1p2/2PPp3/1p2B3/P3PPP1/2KR1BNR w - - 0 16",
            board.into_fen()
        );
    }
}
