mod board;
mod piece;
mod square;

use board::Board;
use piece::{Color, Piece, PieceKind};
use square::Square;
use std::io::{self, Write};

fn main() {
    let mut board = Board::new();
    let mut sira = Color::White;

    loop {
        print!("{board}");
        let isim = match sira {
            Color::White => "White",
            Color::Black => "Black",
        };

        print!("{isim} Turn (Press q for Quit.)");
        io::stdout().flush().unwrap();

        let mut girdi = String::new();
        if io::stdin().read_line(&mut girdi).unwrap_or(0) == 0 {
            break;
        }

        let girdi = girdi.trim();

        if girdi == "q" {
            break;
        }

        let parcalar: Vec<&str> = girdi.split_whitespace().collect();
        if parcalar.len() != 2 {
            println!("Type your move like this 'e2 e4'. ");
            continue;
        }

        let (Some(from), Some(to)) = (Square::parse(parcalar[0]), Square::parse(parcalar[1]))
        else {
            println!("Invlaid Move. \n");
            continue;
        };

        match board.get(from.file, from.rank) {
            Some(p) if p.color == sira => {}
            Some(_) => {
                println!("That is not your piece. \n");
                continue;
            }
            None => {
                println!("That square is empty. \n");
            }
        }

        if board.make_move(from, to) {
            sira = sira.opposite();
        } else {
            println!("Move could not be apply. \n");
        };
    }
}
