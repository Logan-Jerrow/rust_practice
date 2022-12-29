use crate::piece::{Color, Kind, Piece};
use core::{fmt::Display, slice::Chunks};

mod movement {
    use crate::piece::Kind;
    use core::str::{self, FromStr};
    use square::Square;

    pub mod square {
        use core::str::{self, FromStr};

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum Square {
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
                    "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" => {
                        Ok(Self::Rank(s.parse::<usize>().unwrap() - 1))
                    }
                    _ => Err(format!("Could not parse '{s}'")),
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn valid_file() {
                assert_eq!("a".parse::<Square>().unwrap(), Square::File(0));
                assert_eq!("b".parse::<Square>().unwrap(), Square::File(1));
                assert_eq!("c".parse::<Square>().unwrap(), Square::File(2));
                assert_eq!("d".parse::<Square>().unwrap(), Square::File(3));
                assert_eq!("e".parse::<Square>().unwrap(), Square::File(4));
                assert_eq!("f".parse::<Square>().unwrap(), Square::File(5));
                assert_eq!("g".parse::<Square>().unwrap(), Square::File(6));
                assert_eq!("h".parse::<Square>().unwrap(), Square::File(7));
            }

            #[test]
            fn valid_rank() {
                assert_eq!("1".parse::<Square>().unwrap(), Square::Rank(0));
                assert_eq!("2".parse::<Square>().unwrap(), Square::Rank(1));
                assert_eq!("3".parse::<Square>().unwrap(), Square::Rank(2));
                assert_eq!("4".parse::<Square>().unwrap(), Square::Rank(3));
                assert_eq!("5".parse::<Square>().unwrap(), Square::Rank(4));
                assert_eq!("6".parse::<Square>().unwrap(), Square::Rank(5));
                assert_eq!("7".parse::<Square>().unwrap(), Square::Rank(6));
                assert_eq!("8".parse::<Square>().unwrap(), Square::Rank(7));
            }

            #[test]
            fn invalid_sqaure() {
                assert!("i".parse::<Square>().is_err());
                assert!("\n".parse::<Square>().is_err());
                assert!("!".parse::<Square>().is_err());
                assert!("9".parse::<Square>().is_err());
                assert!("0".parse::<Square>().is_err());
                assert!("00324".parse::<Square>().is_err());
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
        fn parse_notation_len2() {
            assert_eq!(
                "g5".parse::<Movement>(),
                Ok(Movement {
                    piece: Kind::Pawn,
                    ambiguitie: None,
                    file: Square::File(6),
                    rank: Square::Rank(4)
                })
            );
        }

        #[test]
        fn parse_notation_len3() {
            assert_eq!(
                "Qg5".parse::<Movement>(),
                Ok(Movement {
                    piece: Kind::Queen,
                    ambiguitie: None,
                    file: Square::File(6),
                    rank: Square::Rank(4)
                })
            );
        }

        #[test]
        fn parse_notation_len4() {
            assert_eq!(
                "B8g5".parse::<Movement>(),
                Ok(Movement {
                    piece: Kind::Bishop,
                    ambiguitie: Some(Square::Rank(7)),
                    file: Square::File(6),
                    rank: Square::Rank(4)
                })
            );

            assert_eq!(
                "Bhg5".parse::<Movement>(),
                Ok(Movement {
                    piece: Kind::Bishop,
                    ambiguitie: Some(Square::File(7)),
                    file: Square::File(6),
                    rank: Square::Rank(4)
                })
            );
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
