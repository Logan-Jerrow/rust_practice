use board::Board;

pub mod board {
    use crate::piece::*;
    use std::slice::Chunks;

    #[derive(Debug)]
    pub struct Board(pub [Option<Piece>; 64]);

    macro_rules! piece {
        (p) => {
            Some(Piece::new(Color::White, PieceKind::Pawn))
        };
        (r) => {
            Some(Piece::new(Color::White, PieceKind::Rook))
        };
        (b) => {
            Some(Piece::new(Color::White, PieceKind::Bishop))
        };
        (h) => {
            Some(Piece::new(Color::White, PieceKind::Knight))
        };
        (q) => {
            Some(Piece::new(Color::White, PieceKind::Queen))
        };
        (k) => {
            Some(Piece::new(Color::White, PieceKind::King))
        };

        (P) => {
            Some(Piece::new(Color::Black, PieceKind::Pawn))
        };
        (R) => {
            Some(Piece::new(Color::Black, PieceKind::Rook))
        };
        (B) => {
            Some(Piece::new(Color::Black, PieceKind::Bishop))
        };
        (H) => {
            Some(Piece::new(Color::Black, PieceKind::Knight))
        };
        (Q) => {
            Some(Piece::new(Color::Black, PieceKind::Queen))
        };
        (K) => {
            Some(Piece::new(Color::Black, PieceKind::King))
        };
        (.) => {
            None
        };
    }

    macro_rules! board {
        [$($piece:tt) *] => {
            Board([
            $(
                // dbg!($piece);
                piece!($piece),
            )*
            ])
        };
    }

    impl Default for Board {
        fn default() -> Self {
            board![
                R H B Q K B H R
                P P P P P P P P
                . . . . . . . .
                . . . . . . . .
                . . . . . . . .
                . . . . . . . .
                p p p p p p p p
                r h b q k b h r
            ]
        }
    }

    impl Board {
        pub fn new() -> Self {
            Self::default()
        }
        pub(crate) fn row(&self) -> Chunks<Option<Piece>> {
            self.0.chunks(8)
        }
        pub fn print(&self) {
            for (i, row) in self.row().enumerate() {
                print!("{}|", i + 1);

                for p in row.iter() {
                    if let Some(p) = p {
                        print!("{p}")
                    } else {
                        print!(".");
                    }
                    print!(" ")
                }
                println!();
            }
            let column = "a|b|c|d|e|f|g|h";
            let h_sep = "—".repeat(column.len());
            println!("  {h_sep}\n  {column}");
        }
    }
}

pub mod piece {
    use std::fmt::Display;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Color {
        White,
        Black,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum PieceKind {
        Pawn,
        Knight,
        Bishop,
        Rook,
        Queen,
        King,
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Piece {
        color: Color,
        piece_kind: PieceKind,
    }

    impl Piece {
        pub fn new(color: Color, piece_kind: PieceKind) -> Self {
            Self { color, piece_kind }
        }
    }

    impl Display for Piece {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.color == Color::Black {
                match &self.piece_kind {
                    PieceKind::Pawn => write!(f, "♟")?,
                    PieceKind::Knight => write!(f, "♞")?,
                    PieceKind::Bishop => write!(f, "♝")?,
                    PieceKind::Rook => write!(f, "♜")?,
                    PieceKind::Queen => write!(f, "♛")?,
                    PieceKind::King => write!(f, "♚")?,
                }
            } else {
                match &self.piece_kind {
                    PieceKind::Pawn => write!(f, "♙")?,
                    PieceKind::Knight => write!(f, "♘")?,
                    PieceKind::Bishop => write!(f, "♗")?,
                    PieceKind::Rook => write!(f, "♖")?,
                    PieceKind::Queen => write!(f, "♕")?,
                    PieceKind::King => write!(f, "♔")?,
                }
            }

            Ok(())
        }
    }
}
fn main() {
    let board = Board::new();
    board.print();
}
