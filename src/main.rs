extern crate rand;

// Package used to generate random numbers
use rand::Rng;
// Package is used for input and output operations
use std::io;
// Package is used to perform operations on a value depending on its position from it's comparision.
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    // 1..200 represents a simple range from 1 to 200.
    let random:i32 = rand::thread_rng().gen_range(1..101);
    loop{
        let mut guess:String = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line!");
        // This statement converts a number from a string into an integer, the expression ::<i32> handles the type to convert to.
        let guess:i32 = guess.trim().parse::<i32>().expect("Please input a number!");
        // match compares two values.
        match guess.cmp(&random) {
           Ordering::Less => println!("Too small!"),
           Ordering::Greater => println!("Too big!"),
           Ordering::Equal => {
            println!("You win!");
            break;
           }
       }
    }
}
