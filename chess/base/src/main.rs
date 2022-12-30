#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod board;
mod piece;

use board::Board;

fn main() {
    let board = Board::new();
    // board.print();
    Board::print_main();
    // Board::print_test();
    // Board::print_left_sidebar();
    // Board::print_emptyboard();
    // Board::print_right_sidebar();
}
