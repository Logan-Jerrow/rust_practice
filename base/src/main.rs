#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod board;
mod piece;

use board::Board;

fn main() {
    let board = Board::new();
    board.print();
}
