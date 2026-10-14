mod board;
mod piece;

use board::Board;
use piece::{Color, Piece, PieceKind};

fn main() {
    let a = Piece {
        color: Color::White,
        kind: PieceKind::Knight,
    };
    let b = Piece {
        color: Color::Black,
        kind: PieceKind::King,
    };

    println!("{}", a.symbol());
    println!("{}", b.symbol());

    let board = Board::new();
    println!("{board}");
}
