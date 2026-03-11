use std::io;

fn main() {
    let mut board = [[' '; 3]; 3];
    let mut player = true;
    while !game_end(&board) {
        print_board(&board);
        let marker = if player { 'X' } else { 'O' };
        loop {
            let mut user_input = String::new();
            let cmd: char; // place for the command
            println!("Wskaż wolne pole (gracz {})", marker);
            let _ = io::stdin().read_line(&mut user_input); // get string from the user input
            cmd = user_input.chars().nth(0).unwrap(); // get the first char from the given string
            let choice = match cmd.to_digit(10) {
                Some(x) if (1..10).contains(&x) => x,
                _ => {
                    println!("Nieprawidłowe dane");
                    continue;
                }
            };
            if board[((choice - 1) / 3) as usize][((choice - 1) % 3) as usize] != ' ' {
                println!("Pole jest zajęte")
            } else {
                board[((choice - 1) / 3) as usize][((choice - 1) % 3) as usize] = marker;
                break;
            }
        }
        player = !player;
    }
    println!("Koniec Gry!");
    let message = match get_winner(&board) {
        None => String::from("Remis"),
        Some(x) => {
            let mut res = String::from("Wygrał gracz ze znacznikiem ");
            res.push(x);
            res
        }
    };
    print_board(&board);
    println!("{}", message);
}

fn print_board(board: &[[char; 3]; 3]) {
    for (i, row) in board.iter().enumerate() {
        println!(
            "{} | {} | {}",
            if row[0] == ' ' {
                std::char::from_digit((3 * i + 1) as u32, 10).unwrap()
            } else {
                row[0]
            },
            if row[1] == ' ' {
                std::char::from_digit((3 * i + 2) as u32, 10).unwrap()
            } else {
                row[1]
            },
            if row[2] == ' ' {
                std::char::from_digit((3 * i + 3) as u32, 10).unwrap()
            } else {
                row[2]
            },
        );
        if i == 2 {
            break;
        }
        println!("- - - - - ");
    }
    println!();
}

fn game_end(board: &[[char; 3]; 3]) -> bool {
    for row in board {
        if row[0] == row[1] && row[1] == row[2] && row[0] != ' ' {
            return true;
        }
    }
    for i in 0..board.len() {
        if board[0][i] == board[1][i] && board[1][i] == board[2][i] && board[0][i] != ' ' {
            return true;
        }
    }
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0] != ' ' {
        return true;
    }
    if board[2][0] == board[1][1] && board[1][1] == board[0][2] && board[2][0] != ' ' {
        return true;
    }

    for row in board {
        for cell in row {
            if *cell == ' ' {
                return false;
            }
        }
    }
    true
}

fn get_winner(board: &[[char; 3]; 3]) -> Option<char> {
    for row in board {
        if row[0] == row[1] && row[1] == row[2] && row[0] != ' ' {
            return Some(row[0]);
        }
    }
    for i in 0..board.len() {
        if board[0][i] == board[1][i] && board[1][i] == board[2][i] && board[0][i] != ' ' {
            return Some(board[0][i]);
        }
    }
    if board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0] != ' ' {
        return Some(board[0][0]);
    }
    if board[2][0] == board[1][1] && board[1][1] == board[0][2] && board[2][0] != ' ' {
        return Some(board[2][0]);
    }

    for row in board {
        for cell in row {
            if *cell == ' ' {
                return None;
            }
        }
    }
    None
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn is_end_vertical(){
        let board = [
            ['X','X','X'],
            ['O','X','O'],
            ['O','O','X']
        ];
        let ans = game_end(&board);
        assert_eq!(true,ans);
    }

    #[test]
    fn is_end_horizontal(){
        let board = [
            ['O','O','O'],
            ['X','X','O'],
            ['X','O','X']
        ];
        let ans = game_end(&board);
        assert_eq!(true,ans);
    }

    #[test]
    fn is_end_diagonal(){
        let board = [
            ['O','X','O'],
            ['X','O','X'],
            ['X','O','O']
        ];
        let ans = game_end(&board);
        assert_eq!(true,ans);
    }


    #[test]
    fn get_winner_vertical(){
        let board = [
            ['X','X','X'],
            ['O','X','O'],
            ['O','O','X']
        ];
        let ans = get_winner(&board);
        assert_eq!('X',ans.unwrap());
    }

    #[test]
    fn get_winner_horizontal(){
        let board = [
            ['O','O','O'],
            ['X','X','O'],
            ['X','O','X']
        ];
        let ans = get_winner(&board);
        assert_eq!('O',ans.unwrap());
    }

    #[test]
    fn get_winner_diagonal(){
        let board = [
            ['O','X','O'],
            ['X','O','X'],
            ['X','O','O']
        ];
        let ans = get_winner(&board);
        assert_eq!('O',ans.unwrap());
    }

    #[test]
    fn get_winner_none(){
        let board = [
            ['O','X','O'],
            ['X','O','X'],
            ['X','O','X']
        ];
        let ans = get_winner(&board);
        assert_eq!(None,ans);
    }
}

