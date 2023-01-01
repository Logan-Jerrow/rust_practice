#![warn(
    clippy::all,
    clippy::complexity,
    clippy::correctness,
    clippy::perf,
    clippy::style,
    clippy::pedantic,
    clippy::nursery
)]

mod board;
mod piece;
mod point {
    pub struct Point(u16, u16);

    // impl From<(usize, usize)> for Point {}
}
mod game {
    use std::io::{stdin, stdout, Write};

    use crate::{board::Board, piece::Color};

    #[derive(Debug, Default)]
    pub struct Game {
        turn: Color,
        board: Board,
    }

    impl Game {
        pub fn play(&mut self) {
            let mut flag: bool = true;
            // while self.board.win() // TODO: check win stat (should board or game do it?)
            while flag {
                self.board.print();

                let mut notation = String::with_capacity(10);
                print!("{} move >>> ", self.turn);
                stdout().flush().unwrap();
                stdin().read_line(&mut notation).unwrap();
                let notation = notation.trim();
                if notation == "q" {
                    println!("END");
                    break;
                }

                let Ok(thing) = self.board.move_piece(self.turn, notation) else {
                    println!("ERROR");
                    break
                };

                // println!(
                //     "{} {} moves to ({}, {})",
                //     self.turn, thing.0, thing.1, thing.2
                // );

                std::thread::sleep(std::time::Duration::from_secs(5));
                self.next_turn();
            }
        }
        fn next_turn(&mut self) {
            self.turn.flip();
        }
    }
}

use game::Game;

fn main() {
    let mut game = Game::default();
    game.play();
}
