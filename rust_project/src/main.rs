//Mini Project: The Guessing Game
//Build a CLI application that generates a random number between 1 and 100. Prompt the user for a guess, read the standard input, parse the string into an integer, and tell the user if their guess is too high, too low, or correct. Keep looping until they win.

/*
1. user input 
2. string to integer, if string
3. compare with rand num - e = r - ip
4. print e = if high , low, correct
5. total while loop
6. rand num generate 
*/

use std::io;
use rand::Rng;

fn main() {

    let temp: u32 = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please enter a number:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");

        let n: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid number entered.");
                continue;
            }
        };

        if n == temp {
            println!("You win!");
            break;
        }
        else if n < temp {
            println!("Too low!");
        }
        else {
            println!("Too high!");
        }
    }
}