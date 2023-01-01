#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Square(pub(crate) char);

impl Square {
    fn is_rank(&self) -> bool {
        self.0.is_numeric()
    }

    fn is_file(&self) -> bool {
        self.0.is_alphabetic()
    }
}

impl TryFrom<char> for Square {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a'..='h' | '1'..='8' => Ok(Self(value)),
            _ => Err(format!("Could not parse '{value}'")),
        }
    }
}

impl std::ops::Deref for Square {
    type Target = char;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct File(pub(crate) u8);
impl TryFrom<char> for File {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a' => Ok(Self(0)),
            'b' => Ok(Self(1)),
            'c' => Ok(Self(2)),
            'd' => Ok(Self(3)),
            'e' => Ok(Self(4)),
            'f' => Ok(Self(5)),
            'g' => Ok(Self(6)),
            'h' => Ok(Self(7)),
            _ => Err(format!("Could not parse '{value}' into file.")),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Rank(pub(crate) u8);
impl TryFrom<char> for Rank {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            r @ '1'..='8' => {
                let digit = r.to_digit(10).unwrap();
                let digit: u8 = digit.try_into().unwrap();
                Ok(Self(digit - 1))
            }
            _ => Err(format!("Could not parse '{value}' into rank.")),
        }
    }
}

// impl TryFrom<char> for Square {
//     type Error = String;

//     fn try_from(value: char) -> Result<Self, Self::Error> {
//         match value {
//             f @ 'a'..='h' => Ok(Self::SqFile(f.try_into()?)),
//             r @ '1'..='8' => Ok(Self::SqRank(r.try_into()?)),
//             _ => Err(format!("Could not parse '{value}'")),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_file() {
        assert_eq!(File::try_from('a'), Ok(File(0)));
        assert_eq!(File::try_from('b'), Ok(File(1)));
        assert_eq!(File::try_from('c'), Ok(File(2)));
        assert_eq!(File::try_from('d'), Ok(File(3)));
        assert_eq!(File::try_from('e'), Ok(File(4)));
        assert_eq!(File::try_from('f'), Ok(File(5)));
        assert_eq!(File::try_from('g'), Ok(File(6)));
        assert_eq!(File::try_from('h'), Ok(File(7)));

        assert!(File::try_from('A').is_err());
        assert!(File::try_from('j').is_err());
        assert!(File::try_from('1').is_err());
    }

    #[test]
    fn tryfrom_rank() {
        assert_eq!(Rank::try_from('1'), Ok(Rank(0)));
        assert_eq!(Rank::try_from('2'), Ok(Rank(1)));
        assert_eq!(Rank::try_from('3'), Ok(Rank(2)));
        assert_eq!(Rank::try_from('4'), Ok(Rank(3)));
        assert_eq!(Rank::try_from('5'), Ok(Rank(4)));
        assert_eq!(Rank::try_from('6'), Ok(Rank(5)));
        assert_eq!(Rank::try_from('7'), Ok(Rank(6)));
        assert_eq!(Rank::try_from('8'), Ok(Rank(7)));

        assert!(Rank::try_from('0').is_err());
        assert!(Rank::try_from('9').is_err());
        assert!(Rank::try_from('a').is_err());
    }

    #[test]
    fn sqaure() {
        assert_eq!(Square::try_from('a'), Ok(Square('a')));
        assert_eq!(Square::try_from('h'), Ok(Square('h')));

        assert_eq!(Square::try_from('1'), Ok(Square('1')));
        assert_eq!(Square::try_from('8'), Ok(Square('8')));

        assert!(Square::try_from('i').is_err());
        assert!(Square::try_from('\n').is_err());
        assert!(Square::try_from('!').is_err());
        assert!(Square::try_from('9').is_err());
        assert!(Square::try_from('0').is_err());
    }

    #[test]
    fn sqaure_file() {
        assert!(Square('a').is_file());
        assert!(Square('h').is_file());

        assert!(!Square('1').is_file());
        assert!(!Square('8').is_file());
    }

    #[test]
    fn sqaure_rank() {
        assert!(Square('1').is_rank());
        assert!(Square('8').is_rank());

        assert!(!Square('a').is_rank());
        assert!(!Square('e').is_rank());
    }
}
