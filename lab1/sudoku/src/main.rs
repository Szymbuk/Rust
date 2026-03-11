fn main() {
    let board = [
        [5, 3, 0, 0, 7, 0, 0, 0, 0],
        [6, 0, 0, 1, 9, 5, 0, 0, 0],
        [0, 9, 8, 0, 0, 0, 0, 6, 0],
        [8, 0, 0, 0, 6, 0, 0, 0, 3],
        [4, 0, 0, 8, 0, 3, 0, 0, 1],
        [7, 0, 0, 0, 2, 0, 0, 0, 6],
        [0, 6, 0, 0, 0, 0, 2, 8, 0],
        [0, 0, 0, 4, 1, 9, 0, 0, 5],
        [0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];
    println!("{}", check_sudoku_board(&board));
}

fn check_sudoku_board(board: &[[i32; 9]; 9]) -> bool {
    check_horizontally(board) && check_vertically(board) && check_squares(board)
}

fn check_horizontally(board: &[[i32; 9]; 9]) -> bool {
    for row in board {
        let mut counters = [0; 10];
        for value in row {
            counters[*value as usize] += 1;
            if *value != 0 && counters[*value as usize] > 1 {
                return false;
            }
        }
    }
    true
}

fn check_vertically(board: &[[i32; 9]; 9]) -> bool {
    for col in 0..board.len() {
        let mut counters = [0; 10];
        for row in board {
            let value: usize = row[col] as usize;
            counters[value] += 1;
            if value != 0 && counters[value] > 1 {
                return false;
            }
        }
    }
    true
}

fn check_squares(board: &[[i32; 9]; 9]) -> bool {
    for square in 0..9 {
        let mut counters = [0; 10];
        for i in 0..3 {
            for j in 0..3 {
                let value = board[(square / 3) * 3 + i][(square % 3) + j];
                counters[value as usize] += 1;
                if value != 0 && counters[value as usize] > 1 {
                    return false;
                }
            }
        }
    }
    true
}
