pub mod board;

use board::Board;
use device_query::{DeviceQuery, DeviceState, Keycode};
use device_query::Keycode::*;

fn main() {
    let board_array = [[0, 9, 4, 8, 1, 2, 3, 0, 0], [0, 3, 0, 7, 0, 0, 1, 9, 0], [1, 5, 0, 0, 9, 6, 0, 8, 0],
        [0, 3, 0, 0, 0, 8, 0, 0, 6], [9, 0, 4, 6, 1, 3, 2, 0, 0], [6, 7, 0, 0, 4, 9, 0, 3, 1],
        [4, 0, 3, 5, 0, 0, 0, 6, 0], [5, 0, 0, 0, 2, 0, 0, 0, 8], [0, 0, 8, 7, 0, 0, 4, 1, 5]];
    let mut board = Board::default();
    board.board = board_array;

    let empty_board = board.clone();
    let solved_boards = board.clone().solve();
    
    let device_state = DeviceState::new();
    
    let mut pointer_position: (isize, isize) = (9 / 2, 9 / 2);
    print!("\x1B[2J\x1B[1;1H");
    board.print_board(pointer_position);

    let mut previous_keys_held: Vec<Keycode> = vec![];
    let mut previous_pointer_position = pointer_position;
    let mut previous_board = board.clone();
    loop {
        let keyboard_state = device_state.get_keys();
        let keys_released: Vec<&Keycode> = keyboard_state.iter().filter(|key| !previous_keys_held.contains(*key)).collect();
        
        for key in keys_released {
            let x = pointer_position.0 as usize;
            let y = pointer_position.1 as usize;
            match key {
                Up => {
                    if [0, 1, 2].contains(&pointer_position.1) {
                        if [0, 1, 2].contains(&pointer_position.0) {
                            continue;
                        }

                        pointer_position.0 = pointer_position.0 - 3;
                        pointer_position.1 = pointer_position.1 % 3 + 6;
                    }
                    else {
                        pointer_position.1 -= 3;
                    }
                },
                Down => {
                    if [6, 7, 8].contains(&pointer_position.1) {
                        if [6, 7, 8].contains(&pointer_position.0) {
                            continue;
                        }

                        pointer_position.0 = pointer_position.0 + 3;
                        pointer_position.1 = pointer_position.1 % 3;
                    }
                    else {
                        pointer_position.1 += 3;
                    }
                },
                Left => {
                    if [0, 3, 6].contains(&pointer_position.1) {
                        if [0, 3, 6].contains(&pointer_position.0) {
                            continue;
                        }

                        pointer_position.0 = pointer_position.0 - 1;
                        pointer_position.1 += 2;
                    }
                    else {
                        pointer_position.1 -= 1;
                    }
                },
                Right => {
                    if [2, 5, 8].contains(&pointer_position.1) {
                        if [2, 5, 8].contains(&pointer_position.0) {
                            continue;
                        }

                        pointer_position.0 = pointer_position.0 + 1;
                        pointer_position.1 -= 2;
                    }
                    else {
                        pointer_position.1 += 1;
                    }
                },
                Backspace => board.board[x][y] = 0,
                Key1 | Numpad1 => board.board[x][y] = 1,
                Key2 | Numpad2 => board.board[x][y] = 2,
                Key3 | Numpad3 => board.board[x][y] = 3,
                Key4 | Numpad4 => board.board[x][y] = 4,
                Key5 | Numpad5 => board.board[x][y] = 5,
                Key6 | Numpad6 => board.board[x][y] = 6,
                Key7 | Numpad7 => board.board[x][y] = 7,
                Key8 | Numpad8 => board.board[x][y] = 8,
                Key9 | Numpad9 => board.board[x][y] = 9,
                _ => ()
            }
        }

        if pointer_position.0 < 0 {
            pointer_position.0 = 0;
        } else if pointer_position.0 > 8 {
            pointer_position.0 = 8;
        }

        if pointer_position.1 < 0 {
            pointer_position.1 = 0;
        } else if pointer_position.1 > 8 {
            pointer_position.1 = 8;
        }

        

        
        if previous_pointer_position != pointer_position || previous_board.board != board.board {
            
            for (i, square) in empty_board.board.iter().enumerate() {
                for (j, field) in square.iter().enumerate() {
                    if field != &0 {
                        board.board[i][j] = *field;
                    }
                }
            } 

            if solved_boards.contains(&board.board) {
                break;
            }

            print!("\x1B[2J\x1B[1;1H");
            board.print_board(pointer_position);
        }
            
        previous_keys_held = keyboard_state;
        previous_pointer_position = pointer_position;
        previous_board = board.clone();
    }

    println!("You won!");
}


