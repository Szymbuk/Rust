use std::io;

fn main()
{
    let mut user_input = String::new();
    let cmd: &str; // place for the command
    println!("Podaj numer ISBN:");
    let _ = io::stdin().read_line(&mut user_input); // get string from the user input
    cmd = user_input.as_str().trim();
    println!("{}",is_isbn(cmd));
}


fn is_isbn(isbn: &str) -> bool{
    let mut sum = 0;
    for letter in isbn.chars(){
        if letter == '-'{
            continue;
        }
        else if letter == 'X'{
            sum+=10;
        }
        else {
            let digit = letter.to_digit(10);
            match digit{
                Some(x) => {
                    sum += x;
                }
                _ => println!("Nie udało się sparsować znaku")
            };

        }
    }
    sum%11 ==0
}