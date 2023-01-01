use super::square::{File, Rank, Square};
use core::str::{self, FromStr};

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Notation {
    pub piece: crate::piece::Kind,
    pub ambiguitie: Option<Square>,
    pub file: File,
    pub rank: Rank,
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
                file: file.try_into()?,
                rank: rank.try_into()?,
                ..Default::default()
            }),
            [Some(piece), Some(file), Some(rank), None] => Ok(Self {
                piece: piece.to_string().parse()?,
                file: file.try_into()?,
                rank: rank.try_into()?,
                ..Default::default()
            }),

            [Some(piece), Some(ambiguitie), Some(file), Some(rank)] => Ok(Self {
                piece: piece.to_string().parse()?,
                ambiguitie: Some(ambiguitie.try_into()?),
                file: file.try_into()?,
                rank: rank.try_into()?,
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
                file: File(6),
                rank: Rank(4),
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
                file: File(6),
                rank: Rank(4),
            })
        );
    }

    #[test]
    fn parse_notation_len4() {
        assert_eq!(
            "B8g5".parse::<Notation>(),
            Ok(Notation {
                piece: Kind::Bishop,
                ambiguitie: Some(Square('8')),
                file: File(6),
                rank: Rank(4),
            })
        );

        assert_eq!(
            "Bhg5".parse::<Notation>(),
            Ok(Notation {
                piece: Kind::Bishop,
                ambiguitie: Some(Square('h')),
                file: File(6),
                rank: Rank(4),
            })
        );
    }
}
