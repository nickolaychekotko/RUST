#![warn(clippy::all, clippy::pedantic)]
use std::io;

fn main() {
    let mut user_choice = String::new();
    
    io::stdin().read_line(&mut user_choice).unwrap();

    let n_choice = process(&user_choice);
    println!("{n_choice}");
    println!("{user_choice}");
}

fn process(str: &String) -> u8
{
    str.trim().parse::<u8>().expect("PLS type a number!")
}
