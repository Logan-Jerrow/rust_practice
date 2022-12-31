use core::{fmt::Display, str::FromStr};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    #[default]
    White,
    Black,
}

impl Color {
    pub fn flip(&mut self) {
        match self {
            Color::White => *self = Color::Black,
            Color::Black => *self = Color::White,
        }
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::White => write!(f, "White"),
            Self::Black => write!(f, "Black"),
        }
    }
}

const BLACK_PAWN: char = '♟';
const BLACK_KNIGHT: char = '♞';
const BLACK_BISHOP: char = '♝';
const BLACK_ROOK: char = '♜';
const BLACK_QUEEN: char = '♛';
const BLACK_KING: char = '♚';

const WHITE_PAWN: char = '♙';
const WHITE_KNIGHT: char = '♘';
const WHITE_BISHOP: char = '♗';
const WHITE_ROOK: char = '♖';
const WHITE_QUEEN: char = '♕';
const WHITE_KING: char = '♔';

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Kind {
    #[default]
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Pawn => write!(f, "Pawn"),
            Self::Knight => write!(f, "Knight"),
            Self::Bishop => write!(f, "Bishop"),
            Self::Rook => write!(f, "Rook"),
            Self::Queen => write!(f, "Queen"),
            Self::King => write!(f, "King"),
        }
    }
}

impl FromStr for Kind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "n" | "N" => Ok(Self::Knight),
            "b" | "B" => Ok(Self::Bishop),
            "r" | "R" => Ok(Self::Rook),
            "q" | "Q" => Ok(Self::Queen),
            "k" | "K" => Ok(Self::King),
            e => Err(format!("Unknown notation: '{e}'")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    color: Color,
    kind: Kind,
}

impl Piece {
    #[must_use]
    pub const fn new(color: Color, piece_kind: Kind) -> Self {
        Self {
            color,
            kind: piece_kind,
        }
    }
}

impl Piece {
    const fn symbol(self) -> char {
        match self.color {
            Color::White => match self.kind {
                Kind::Pawn => WHITE_PAWN,
                Kind::Knight => WHITE_KNIGHT,
                Kind::Bishop => WHITE_BISHOP,
                Kind::Rook => WHITE_ROOK,
                Kind::Queen => WHITE_QUEEN,
                Kind::King => WHITE_KING,
            },
            Color::Black => match self.kind {
                Kind::Pawn => BLACK_PAWN,
                Kind::Knight => BLACK_KNIGHT,
                Kind::Bishop => BLACK_BISHOP,
                Kind::Rook => BLACK_ROOK,
                Kind::Queen => BLACK_QUEEN,
                Kind::King => BLACK_KING,
            },
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Color, Kind, Piece, BLACK_KING};

    #[test]
    fn symbol() {
        assert_eq!(Piece::new(Color::Black, Kind::King).symbol(), BLACK_KING);
    }

    #[test]
    fn parse_piecekind() {
        assert_eq!("n".parse::<Kind>(), Ok(Kind::Knight));
        assert_eq!("b".parse::<Kind>(), Ok(Kind::Bishop));
        assert_eq!("r".parse::<Kind>(), Ok(Kind::Rook));
        assert_eq!("q".parse::<Kind>(), Ok(Kind::Queen));
        assert_eq!("k".parse::<Kind>(), Ok(Kind::King));

        assert_eq!("N".parse::<Kind>(), Ok(Kind::Knight));
        assert_eq!("B".parse::<Kind>(), Ok(Kind::Bishop));
        assert_eq!("R".parse::<Kind>(), Ok(Kind::Rook));
        assert_eq!("Q".parse::<Kind>(), Ok(Kind::Queen));
        assert_eq!("K".parse::<Kind>(), Ok(Kind::King));
    }
}
