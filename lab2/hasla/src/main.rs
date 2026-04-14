use std::io;
use rand::{rng, Rng};





fn main()
{
    loop {
        let mut user_input = String::new();
        let cmd: &str; // place for the command
        println!("Podaj długość generowanego hasła:");
        let _ = io::stdin().read_line(&mut user_input); // get string from the user input
        cmd = user_input.as_str().trim();
        let n:usize = match cmd.parse() {
            Ok(x) => x,
            Err(e) => {
                println!("{:?}",e);
                continue;
            }
        };
        let mut user_input = String::new();
        let cmd: &str; // place for the command
        println!("Podaj zakresy generowanych znaków (lowercase,uppercase,digits,special):");
        let _ = io::stdin().read_line(&mut user_input); // get string from the user input
        cmd = user_input.as_str().trim();
        let mut iterator = cmd.split(' ');
        let charset :[&str;4] =[
            iterator.next().unwrap_or(""),
            iterator.next().unwrap_or(""),
            iterator.next().unwrap_or(""),
            iterator.next().unwrap_or("")
        ];
        for _ in 0..5 {
            //let password = generate_password1(n);
            let password = generate_password2(n, &charset);
            println!("{password}");
        }
        break
    };

}


fn generate_password1(n: usize) -> String{
    let mut rng = rand::rng();

    let legal_characters = ['1','2','3','4','5','6','7','8','9','0',
        'q','w','e','r','t','y','u','i','o','p','a','s','d','f','g','h','j','k','l','z','x','c','v','b','n','m',
        'Q','W','E','R','T','Y','U','I','O','P','A','S','D','F','G','H','J','K','L','Z','X','C','V','B','N','M',
        '!','@','#','$','%','^','&','*','?'];
    let mut result = String::from("");
    for _ in 0..n{
        let ch =     legal_characters[rng.random_range(0..legal_characters.len())];
        result.push(ch);
    }
    result
}


fn generate_password2(n: usize,charsets:&[&str]) -> String{
    let mut rng = rand::rng();

    let lowercase = ['q','w','e','r','t','y','u','i','o','p','a','s','d','f','g','h','j','k','l','z','x','c','v','b','n','m'];
    let uppercase = ['Q','W','E','R','T','Y','U','I','O','P','A','S','D','F','G','H','J','K','L','Z','X','C','V','B','N','M'];
    let digits = ['1','2','3','4','5','6','7','8','9','0'];
    let special = ['!','@','#','$','%','^','&','*','?'];

    let mut final_chars:Vec<char> = Vec::new();
    if charsets.contains(&"lowercase"){
        final_chars.extend(lowercase.iter());
    }
    if charsets.contains(&"uppercase"){
        final_chars.extend(uppercase.iter());
    }
    if charsets.contains(&"digits"){
        final_chars.extend(digits.iter());
    }
    if charsets.contains(&"special"){
        final_chars.extend(special.iter());
    }
    if final_chars.len()== 0{
        final_chars.extend(lowercase.iter());
        final_chars.extend(uppercase.iter());
        final_chars.extend(digits.iter());
        final_chars.extend(special.iter())
    }

    let mut result = String::from("");
    for _ in 0..n{
        let ch =     final_chars[rng.random_range(0..final_chars.len())];
        result.push(ch);
    }
    result
}


#[cfg(test)]
mod tests{
    use super::*;


    const N: usize =1;
    const LOWERCASE: [char; 26] = ['q','w','e','r','t','y','u','i','o','p','a','s','d','f','g','h','j','k','l','z','x','c','v','b','n','m'];
    const UPPERCASE: [char; 26] = ['Q','W','E','R','T','Y','U','I','O','P','A','S','D','F','G','H','J','K','L','Z','X','C','V','B','N','M'];
    const DIGITS: [char; 10] = ['1','2','3','4','5','6','7','8','9','0'];
    const SPECIAL: [char; 9] = ['!','@','#','$','%','^','&','*','?'];

    #[test]
    fn generate_password2_lowercase(){
        let charset = ["lowercase"];
        let result = generate_password2(N,&charset);

        for letter in result.as_bytes(){
            assert_eq!(LOWERCASE.contains(&(*letter as char)),true);
        }
    }

    #[test]
    fn generate_password2_uppercase(){
        let charset = ["uppercase"];
        let result = generate_password2(N,&charset);

        for letter in result.as_bytes(){
            assert_eq!(UPPERCASE.contains(&(*letter as char)),true);
        }
    }

    #[test]
    fn generate_password2_special(){
        let charset = ["special"];
        let result = generate_password2(N,&charset);

        for letter in result.as_bytes(){
            assert_eq!(SPECIAL.contains(&(*letter as char)),true);
        }
    }

    #[test]
    fn generate_password2_digits(){
        let charset = ["digits"];
        let result = generate_password2(N,&charset);

        for letter in result.as_bytes(){
            assert_eq!(DIGITS.contains(&(*letter as char)),true);
        }
    }

    #[test]
    fn generate_password2_none(){
        let charset = [];
        let result = generate_password2(N,&charset);
        let mut all = Vec::from(UPPERCASE);
        all.extend(LOWERCASE.iter());
        all.extend(DIGITS.iter());
        all.extend(SPECIAL.iter());

        for letter in result.as_bytes(){
            assert_eq!(all.contains(&(*letter as char)),true);
        }
    }


    #[test]
    fn generate_password2_mixed2(){
        let charset = ["uppercase","special"];
        let result = generate_password2(N,&charset);
        let mut all = Vec::from(UPPERCASE);
        all.extend(SPECIAL.iter());

        for letter in result.as_bytes(){
            assert_eq!(all.contains(&(*letter as char)),true);
        }
    }
}

