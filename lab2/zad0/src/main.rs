use std::ops::Add;

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    let mut start = 0;
    let mut end= input.len();
    for (i,letter) in input.as_bytes().iter().enumerate(){
        if *letter != b' '{
            start = i;
            break
        }
    }
    for (i,letter) in input.as_bytes().iter().rev().enumerate(){
        if *letter != b' '{
            end = input.len()-i;
            break
        }
    }
    String::from(&input[start..end])
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    let result = String::from(input);
    result.add(" world!")
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    input.replace("cars","balloons")
}

fn main() {
    assert_eq!(trim_me("Hello!     "), "Hello!");
    assert_eq!(trim_me("  What's up!"), "What's up!");
    assert_eq!(trim_me("   Hola!  "), "Hola!");


    assert_eq!(compose_me("Hello"), "Hello world!");
    assert_eq!(compose_me("Goodbye"), "Goodbye world!");


    assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
    assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
}