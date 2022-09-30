fn main() {
    use std::io::{stdin, stdout, Write};

    let mut input = String::new();
    print!("Enter your name: ");

    let _= stdout().flush();
    stdin().read_line(&mut input).expect("Did not enter a correct string");

    if let Some('\r') = input.chars().next_back() {
        input.pop();
    }

    print!("Hello {}", input);

    let mut score_input = String::new();
    print!("Enter your score: ");

    let _= stdout().flush();
    stdin().read_line(&mut score_input).expect("Did not enter a correct string");

    if let Some('\r') = score_input.chars().next_back() {
        score_input.pop();
    }

    let score: u32 = score_input.trim().parse().expect("Please type a number!");

    print!("Your grade is {}", grade(score));
}

pub fn grade(score: u32) -> char {
    match score {
        0..=59 => 'F',
        60..=69 => 'D',
        70..=79 => 'C',
        80..=89 => 'B',
        90..=100 => 'A',
        _ => panic!("Invalid score: {}", score),
    }
}
