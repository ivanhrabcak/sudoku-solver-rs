
#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub board: [[u8; 9]; 9],
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

        return Board{ board };
    }
}