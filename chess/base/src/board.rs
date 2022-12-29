use crate::piece::{Color, Kind, Piece};
use core::{fmt::Display, slice::Chunks};

mod movement {
    use crate::piece::Kind;
    use core::str::{self, FromStr};

    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Square {
        File(usize),
        Rank(usize),
    }

    impl FromStr for Square {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "a" => Ok(Self::File(0)),
                "b" => Ok(Self::File(1)),
                "c" => Ok(Self::File(2)),
                "d" => Ok(Self::File(3)),
                "e" => Ok(Self::File(4)),
                "f" => Ok(Self::File(5)),
                "g" => Ok(Self::File(6)),
                "h" => Ok(Self::File(7)),
                "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" => {
                    Ok(Self::Rank(s.parse::<usize>().unwrap() - 1))
                }
                _ => Err(format!("Could not parse {s}.")),
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct Movement {
        // player: crate::piece::Color,
        piece: crate::piece::Kind,
        ambiguitie: Option<Square>,
        file: Square,
        rank: Square,
    }

    impl Movement {}

    impl Default for Movement {
        fn default() -> Self {
            Self {
                piece: Kind::default(),
                ambiguitie: Option::default(),
                file: Square::File(0),
                rank: Square::Rank(0),
            }
        }
    }

    impl FromStr for Movement {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let mut characters: [Option<char>; 4] = [None; 4];
            s.chars()
                .zip(characters.iter_mut())
                .for_each(|(c, ptr)| *ptr = Some(c));

            match characters {
                [Some(file), Some(rank), None, None] => Ok(Self {
                    file: file.to_string().parse()?,
                    rank: rank.to_string().parse()?,
                    ..Default::default()
                }),
                [Some(piece), Some(file), Some(rank), None] => Ok(Self {
                    piece: piece.to_string().parse()?,
                    file: file.to_string().parse()?,
                    rank: rank.to_string().parse()?,
                    ..Default::default()
                }),

                [Some(piece), Some(ambiguitie), Some(file), Some(rank)] => Ok(Self {
                    piece: piece.to_string().parse()?,
                    ambiguitie: Some(ambiguitie.to_string().parse()?),
                    file: file.to_string().parse()?,
                    rank: rank.to_string().parse()?,
                }),
                _ => Err("notation wrong length")?,
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn parse_mov() {
            let mov = "Qg5".parse::<Movement>().unwrap();
            assert_eq!(mov.piece, Kind::Queen);
            assert_eq!(mov.file, Square::File(6));
            assert_eq!(mov.rank, Square::Rank(4));
        }
    }
}

#[derive(Debug)]
pub struct Board(pub [Option<Piece>; 64]);

macro_rules! piece {
    (p) => {
        Some(Piece::new(Color::White, Kind::Pawn))
    };
    (r) => {
        Some(Piece::new(Color::White, Kind::Rook))
    };
    (b) => {
        Some(Piece::new(Color::White, Kind::Bishop))
    };
    (h) => {
        Some(Piece::new(Color::White, Kind::Knight))
    };
    (q) => {
        Some(Piece::new(Color::White, Kind::Queen))
    };
    (k) => {
        Some(Piece::new(Color::White, Kind::King))
    };

    (P) => {
        Some(Piece::new(Color::Black, Kind::Pawn))
    };
    (R) => {
        Some(Piece::new(Color::Black, Kind::Rook))
    };
    (B) => {
        Some(Piece::new(Color::Black, Kind::Bishop))
    };
    (H) => {
        Some(Piece::new(Color::Black, Kind::Knight))
    };
    (Q) => {
        Some(Piece::new(Color::Black, Kind::Queen))
    };
    (K) => {
        Some(Piece::new(Color::Black, Kind::King))
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

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer: String = String::with_capacity(281); // TODO: Calculate needed capacity

        let mut row_number = 8_u8;
        for row in self.row() {
            buffer.push_str(&format!("{row_number}|"));
            row_number -= 1;

            for s in row
                .iter()
                .map(|o| o.map_or_else(|| ".".to_string(), |p| p.to_string()))
            {
                buffer.push_str(&s);
                buffer.push(' ');
            }
            buffer.push('\n');
        }
        // buffer.push_str(&"\u{203e}".repeat(15));
        // buffer.push_str(&"_".repeat(15));

        buffer.push_str("  ");
        buffer.push_str(&"—".repeat(15));
        buffer.push_str("  \n  a|b|c|d|e|f|g|h");

        write!(f, "{buffer}")
    }
}

impl Board {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    pub(crate) fn row(&self) -> Chunks<Option<Piece>> {
        self.0.chunks(8)
    }

    pub fn print(&self) {
        println!("{self}");
    }

    pub fn move_piece<I>(mut self, p: usize) {
        // self.0.get_mut(p);

        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn movement() {
    //     let board = Board::default();

    // }
}
