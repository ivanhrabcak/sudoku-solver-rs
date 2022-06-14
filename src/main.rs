pub mod board;

use board::Board;

fn main() {
    let mut board_array = [[0; 9]; 9];
    for line in board_array.iter_mut() {
        let mut x = 1;
        for field in line.iter_mut() {
            *field = x;
            x += 1;
        }
    }

    let board = Board::default();
    println!("{board:?}");
}