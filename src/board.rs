
#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub board: [[u8; 9]; 9],
}

impl Board {
    pub fn line_has_number(&self, n: u8, line_ind: usize) -> bool {
        for i in 0..3usize {
            let square = self.board[(line_ind / 3) as usize + i];
            for i in 0..3usize {
                if square[(line_ind % 3) * 3 + i] == n {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn column_has_number(&self, n: u8, column_ind: usize) -> bool {
        for i in 0..3usize {
            let square: [u8; 9] = self.board[(column_ind / 3) as usize + i];
            for j in 0..3usize {
                if square[3 * j + column_ind % 3] == n {
                    return true;
                }
            }
        }

        return false;
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