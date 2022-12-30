#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod board;
mod piece;

use board::Board;

fn main() {
    let board = Board::new();
    Board::print_sidebar();
    board.print_board();
}
