use crate::piece::{self, Color, Piece, PieceKind};
use std::fmt;

use crate::square::Square;

pub struct Board {
    squares: [[Option<Piece>; 8]; 8],
}

impl Board {
    fn index(file: char, rank: u8) -> Option<(usize, usize)> {
        if !('a'..='h').contains(&file) || !(1..=8).contains(&rank) {
            return None;
        }
        let sutun = (file as u8 - b'a') as usize;
        let satir = (rank - 1) as usize;
        Some((satir, sutun))
    }

    pub fn get(&self, file: char, rank: u8) -> Option<Piece> {
        let (satir, sutun) = Self::index(file, rank)?;
        self.squares[satir][sutun]
    }

    pub fn set(&mut self, file: char, rank: u8, piece: Option<Piece>) {
        if let Some((satir, sutun)) = Self::index(file, rank) {
            self.squares[satir][sutun] = piece;
        }
    }

    pub fn new() -> Self {
        let mut board = Board {
            squares: [[None; 8]; 8],
        };

        for file in 'a'..='h' {
            board.set(
                file,
                2,
                Some(Piece {
                    color: Color::White,
                    kind: PieceKind::Pawn,
                }),
            );
            board.set(
                file,
                7,
                Some(Piece {
                    color: Color::Black,
                    kind: PieceKind::Pawn,
                }),
            );
        }

        let back_site = [
            PieceKind::Rook,
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::Queen,
            PieceKind::King,
            PieceKind::Bishop,
            PieceKind::Knight,
            PieceKind::Rook,
        ];

        for (file, kind) in ('a'..='h').zip(back_site) {
            board.set(
                file,
                1,
                Some(Piece {
                    color: Color::White,
                    kind: kind,
                }),
            );
            board.set(
                file,
                8,
                Some(Piece {
                    color: Color::Black,
                    kind: kind,
                }),
            );
        }
        board
    }

    pub fn make_move(&mut self, from: Square, to: Square) -> bool {
        if from == to {
            return false;
        }

        let piece = match self.get(from.file, from.rank) {
            Some(p) => p,
            None => return false,
        };

        self.set(to.file, to.rank, Some(piece));
        self.set(from.file, from.rank, None);
        true
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for rank in (1..=8).rev() {
            write!(f, "{}", rank)?;

            for file in 'a'..='h' {
                match self.get(file, rank) {
                    Some(piece) => write!(f, "{}", piece.symbol())?,
                    None => write!(f, ".")?,
                }
            }
            writeln!(f)?;
        }
        writeln!(f, "abcdefgh")?;
        Ok(())
    }
}
