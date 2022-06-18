use colored::Colorize;
use terminal_size::terminal_size;


#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub board: [[u8; 9]; 9],
    terminal_size: (u16, u16)
}

impl Board {
    pub fn count_n_in_line(&self, n: u8, line_ind: usize) -> i32 {
        let mut counter = 0;
        for i in 0..3usize {
            let square = self.board[(line_ind / 3) * 3 as usize + i];
            for j in 0..3usize {
                if square[(line_ind % 3) * 3 + j] == n {
                    counter += 1;
                }
            }
        }

        return counter;
    }

    pub fn count_n_in_column(&self, n: u8, column_ind: usize) -> i32 {
        let mut counter = 0;
        for i in 0..3usize {
            let square: [u8; 9] = self.board[(column_ind / 3) as usize + i * 3];
            for j in 0..3usize {
                if square[3 * j + column_ind % 3] == n {
                    counter += 1;
                }
            }
        }

        return counter;
    }

    pub fn get_line_for_pos(&self, x: usize, y: usize) -> usize {
        ((x / 3) * 3) + y / 3
    }

    pub fn get_column_pos(&self, x: usize, y: usize) -> usize {
        ((x % 3) * 3) + y % 3 
    }

    pub fn count_n_in_square(&self, n: u8, square_ind: usize) -> i32 {
        self.board[square_ind].iter().filter(|x| *x == &n).count() as i32
    }

    pub fn is_possible(&self, x: usize, y: usize, n: u8) -> bool {
        return self.count_n_in_square(n, x) == 0
            && self.count_n_in_line(n, self.get_line_for_pos(x, y)) == 0
            && self.count_n_in_column(n, self.get_column_pos(x, y)) == 0;
    }

    pub fn is_board_possible(&mut self) -> bool {
        for (i, square) in self.board.clone().iter().enumerate() {
            for (j, field) in square.iter().enumerate() {
                let original_value = *field;
                self.board[i][j] = 0;
                if !self.is_possible(i, j, original_value) {
                    self.board[i][j] = original_value;
                    return false;
                }

                self.board[i][j] = original_value;
            }
        }

        return true;
    }

    pub fn print_board(&self, cursor_position: (isize, isize)) {
        let mut output = String::new();
        let top_spacing = "\n".repeat(self.terminal_size.1 as usize / 2 - 15 / 2);
        let padding = " ".repeat(self.terminal_size.0 as usize / 2);

        output += &top_spacing;

        fn color(text: &str, x: usize, y: usize, cursor_position: (isize, isize), board: &Board) -> String {
            if board.board[x][y] == board.board[cursor_position.0 as usize][cursor_position.1 as usize] {
                return text.bright_red().to_string();
            } else if x == cursor_position.0 as usize
                || board.get_column_pos(x, y) == board.get_column_pos(cursor_position.0 as usize, cursor_position.1 as usize)
                || board.get_line_for_pos(x, y) == board.get_line_for_pos(cursor_position.0 as usize, cursor_position.1 as usize) {
                return text.cyan().to_string();
            } else {
                return text.to_string();
            }
        }

        output += &padding;
        output += "=========================\n";
        for line in 0..9usize {
            for i in 0..3usize {
                let square = self.board[(line / 3) * 3 as usize + i];
                if i == 0 {
                    output += &padding;
                    output += "|";
                }

                let x = (line / 3) * 3 as usize + i;

                let mut previous_was_cursor = false;
                for j in 0..3usize {
                    let number = square[(line % 3) * 3 + j];
                    
                    let y = (line % 3) * 3 + j;

                    if (x as isize, y as isize) == cursor_position {
                        output += "[";
                        previous_was_cursor = true;
                    } else if previous_was_cursor {
                        output += "]";
                        previous_was_cursor = false;
                    } else {
                        output += " ";
                    }

                    if number == 0 {
                        output += &color(" ", x, y, cursor_position, self);
                    } else {
                        output += &color(&format!("{}", number), x, y, cursor_position, self);
                    }
                }

                if previous_was_cursor {
                    output += "]";
                }
                else {
                    output += " ";
                }

                output += "|";
            }
            output += "\n";
            if line % 3 == 2 {
                output += &padding;
                output += "=========================\n";
            }
        }
        println!("{}", output);
    }

    pub fn is_solved(&mut self) -> bool {
        for (i, square) in self.board.clone().iter().enumerate() {
            for (j, field) in square.iter().enumerate() {
                if field == &0 {
                    return false;
                } 

                let original_value = *field;
                self.board[i][j] = 0;
                if !self.is_possible(i, j, original_value) {
                    self.board[i][j] = original_value;
                    return false;
                }

                self.board[i][j] = original_value;
            }
        }

        return true;
    }

    pub fn solve(&mut self) -> Vec<[[u8; 9]; 9]> {
        if self.is_solved() {
            return vec![self.board.clone()];
        }

        let mut solutions: Vec<[[u8; 9]; 9]> = vec![];

        for (i, square) in self.board.clone().iter().enumerate() {
            if self.count_n_in_square(0, i) == 0 {
                continue;
            }
            for (j, field) in square.iter().enumerate() {
                if field != &0 {
                    continue;
                }

                for possible_n in 1..10 {
                    if self.is_possible(i, j, possible_n) {
                        self.board[i][j] = possible_n;

                        solutions.append(&mut self.solve());

                        self.board[i][j] = 0;
                    } 
                }

                return solutions;
            }
        }

        return solutions;
    }
}


impl Default for Board {
    fn default() -> Self {
        let mut board: [[u8; 9]; 9] = [[0; 9]; 9];

        for line in board.iter_mut() {
            let mut x = 1;
            for field in line.iter_mut() {
                *field = x;
                x += 1;
            }
        }

        let width = if let Some((w, h)) = terminal_size() {
                (w.0, h.0)
            } else {
                panic!("Failed to get terminal size!");
            };

        return Board{ board, terminal_size: width };
    }
}