use chess_state_machine::board::Board;
use chess_state_machine::fen::{FromFen, IntoFen};
use chess_state_machine::square::Square;
use std::convert::TryFrom;

fn main() {
    let mut board = Board::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    // Pawn to D4
    board.play_move(
        Square::try_from("D2").unwrap(),
        Square::try_from("D4").unwrap()
    );

    // Pawn to E5
    board.play_move((4, 6), (4, 4));

    // Pawn takes E5
    board.play_move([3, 3], [4, 4]);

    println!("{:?}", board.into_fen());
    // rnbqkbnr/pppp1ppp/8/4P3/8/8/PPP1PPPP/RNBQKBNR b KQkq - 0 2
}