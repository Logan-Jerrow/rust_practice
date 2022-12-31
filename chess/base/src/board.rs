mod macros;
mod movement;

use self::movement::Notation;
use crate::{
    board,
    piece::{self, Kind, Piece},
};
use itertools::Itertools;
use std::io::{stdout, Write};
use termion::{
    clear,
    cursor::{self, DetectCursorPos, Goto},
    raw::IntoRawMode,
};

#[derive(Debug)]
pub struct Board(pub [Option<Piece>; 64]);

impl Board {
    const FILE_RANK_BAR: &str = "  a|b|c|d|e|f|g|h\r
 +———————————————+\r
8|               |8\r
7|               |7\r
6|               |6\r
5|               |5\r
4|               |4\r
3|               |3\r
2|               |2\r
1|               |1\r
 +———————————————+\r
  a|b|c|d|e|f|g|h";

    const fn start_position(/*s: &str*/) -> Goto {
        /*
                let mut file_row = 0_usize;
                let mut file_col = 0_usize;
                for (r, l) in s.lines().enumerate() {
                    match l.find('a') {
                        Some(c) => {
                            file_row = r;
                            file_col = c;
                            break;
                        }
                        None => continue,
                    }
                }

                let mut rank_row = 0_usize;
                let mut rank_col = 0_usize;
                for (r, l) in s.lines().enumerate() {
                    match l.find('1') {
                        Some(c) => {
                            rank_row = r;
                            rank_col = c;
                            break;
                        }
                        None => continue,
                    }
                }

                println!("file point: ({file_col},{file_row})");
                println!("rank point: ({rank_col},{rank_row})");
        */

        Goto(3, 3)
    }

    pub fn print_sidebar(row_space: u16) -> Goto {
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(stdout, "{}{}", clear::All, cursor::Goto(1, 1)).unwrap();

        write!(stdout, "{}\r\n", Self::FILE_RANK_BAR).unwrap();
        let end = stdout.cursor_pos().unwrap();

        let board_start = Self::start_position(); //Self::start_position(Self::FILE_RANK_BAR);
        let Goto(start_x, mut start_y) = board_start;
        // TODO: range based on row_space
        for _ in 1..=8 {
            write!(stdout, "{}", cursor::Goto(start_x, start_y)).unwrap();
            write!(stdout, ". . . . . . . .").unwrap();

            start_y += row_space;
            write!(stdout, "{}", cursor::Goto(start_x, start_y)).unwrap();
        }
        write!(stdout, "{}", cursor::Goto(end.0, end.1)).unwrap();

        board_start
    }

    pub fn print_board(&self, board_start: Goto, space: (u16, u16)) {
        print!("{}", cursor::Save);

        let Goto(mut x, mut y) = board_start; //(3_u16, 3_u16);
        for row in &self.0.iter().chunks(8) {
            for o in row {
                let p = o.map_or_else(|| ".".to_string(), |p| p.to_string());
                print!("{}{p}", cursor::Goto(x, y));
                x += space.0;
            }
            println!();
            x = board_start.0;
            y += space.1;
        }
        print!("{}", cursor::Restore);
    }

    pub fn print(&self) {
        let space = (2, 1);
        let start = Self::print_sidebar(space.1);
        self.print_board(start, space);
        println!();
        stdout().flush().unwrap();
    }

    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn move_piece(
        &mut self,
        color: piece::Color,
        notation: &str,
    ) -> Result<(Kind, usize, usize), String> {
        let mov = notation.parse::<Notation>()?;
        let dest = (mov.rank, mov.file);

        let piece = Piece::new(color, mov.piece);

        let src = self
            .0
            .into_iter()
            .filter_map(|o| if o.as_ref() == Some(&piece) { o } else { None })
            .collect::<Vec<_>>();

        dbg!(src);

        Ok((mov.piece, mov.file, mov.rank))
    }
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

#[cfg(test)]
mod tests {
    use crate::piece::Color;

    use super::*;

    #[test]
    fn movement() {
        let board = Board::default();
        board.move_piece(Color::Black, "a3");

        // assert_eq!(board.move_piece("a1"), Ok((Kind::Pawn, 0, 0)));
    }

    #[test]
    fn board_start() {
        let Goto(x, y) = Board::start_position(Board::FILE_RANK_BAR);
        assert_eq!((x, y), (3, 3));
    }
}
