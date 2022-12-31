use core::str::{self, FromStr};
use square::Square;

pub mod square {
    use core::str::{self, FromStr};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Square {
        File(usize),
        Rank(usize),
    }

    impl Square {
        pub const fn index(&self) -> usize {
            match *self {
                Self::File(i) | Self::Rank(i) => i,
            }
        }
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Notation {
    // player: crate::piece::Color,
    pub piece: crate::piece::Kind,
    pub ambiguitie: Option<Square>,
    pub file: usize,
    pub rank: usize,
}

impl Notation {}

impl FromStr for Notation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut characters: [Option<char>; 4] = [None; 4];
        s.chars()
            .zip(characters.iter_mut())
            .for_each(|(c, ptr)| *ptr = Some(c));

        match characters {
            [Some(file), Some(rank), None, None] => Ok(Self {
                file: file.to_string().parse::<Square>()?.index(),
                rank: rank.to_string().parse::<Square>()?.index(),
                ..Default::default()
            }),
            [Some(piece), Some(file), Some(rank), None] => Ok(Self {
                piece: piece.to_string().parse()?,
                file: file.to_string().parse::<Square>()?.index(),
                rank: rank.to_string().parse::<Square>()?.index(),
                ..Default::default()
            }),

            [Some(piece), Some(ambiguitie), Some(file), Some(rank)] => Ok(Self {
                piece: piece.to_string().parse()?,
                ambiguitie: Some(ambiguitie.to_string().parse()?),
                file: file.to_string().parse::<Square>()?.index(),
                rank: rank.to_string().parse::<Square>()?.index(),
            }),
            _ => Err("notation wrong length")?,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::Kind;

    #[test]
    fn parse_notation_len2() {
        assert_eq!(
            "g5".parse::<Notation>(),
            Ok(Notation {
                piece: Kind::Pawn,
                ambiguitie: None,
                file: 6,
                rank: 4
            })
        );
    }

    #[test]
    fn parse_notation_len3() {
        assert_eq!(
            "Qg5".parse::<Notation>(),
            Ok(Notation {
                piece: Kind::Queen,
                ambiguitie: None,
                file: 6,
                rank: 4
            })
        );
    }

    #[test]
    fn parse_notation_len4() {
        assert_eq!(
            "B8g5".parse::<Notation>(),
            Ok(Notation {
                piece: Kind::Bishop,
                ambiguitie: Some(Square::Rank(7)),
                file: 6,
                rank: 4
            })
        );

        assert_eq!(
            "Bhg5".parse::<Notation>(),
            Ok(Notation {
                piece: Kind::Bishop,
                ambiguitie: Some(Square::File(7)),
                file: 6,
                rank: 4
            })
        );
    }
}
