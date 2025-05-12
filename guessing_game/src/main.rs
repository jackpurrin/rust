use std::io; // imports the io lib from std, allows for strings and stuff
use std::cmp::Ordering; // imports the ordering lib from std, allows us to compare values to each other
use rand::Rng; // imports the rng lib from the rand library

fn main() { // main function of the program, all the code goes into here to get run
    println!("Guess the number!"); // prints out a single line, "guess the number!" using a rust macro

    println!("Please input your guess."); // same as last

    let mut guess = String::new();
    // sets a variable, guess. normally, variables are `immutable` which means it cannot change. 
    // but if you specify `mut`, it will be mutable, i can change it after inputting it. 
    // the `String::New()` just creates a new empty string that we can change later.

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // defines a new variable, secret_number. it's equal to a random number between 1-100.
    // rand just says use the rand library
    // thread_rng just means that is local to the current thread of execution and is seeded by the operating system
    // gen_range is just the range that the number will be generate from.

    io::stdin() // from the io module, we'll use stdin. stdin allows for user input
        .read_line(&mut guess) 
        // .read_line handles user input, from the keyboard.
        // we pass through `&mut guess` to store the user input. 
        // this will store it in the previously defined `guess` variable. we add the &mut so it knows it's mutable.
        .expect("Failed to read line"); // error handling. if it fails to read, it'll output this.

    let guess: u32 = guess.trim().parse().expect("Please type a number!"); 
    // we need to turn guess into a u32 number value, as we cannot compare a string and a number
    // .trim removes all whitespace at the end and at the start of the string.
    // .parse tries to turn one value type into another. as defined, a string into a u32 value
    // if we cannot turn the string into a u32 value, we're gonna assume the user entered a string not a number
    // so we just say "Please type a number!" as our error handling

    println!("You guessed: {}", guess); 
    // this is the last line of code. this outputs "You guessed: {guess}"
    // but how? the {} indicates that the second param, guess, should be placed here.
    // resulting in "You guessed: {number you guessed}"

    match guess.cmp(&secret_number) { 
    // this is pretty much a function that compares something
    // the guess.cmp part is the variable you wanna compare to the other input. {variable}.cmp({the varibale you wanna compare})
        Ordering::Less => println!("Too small!"), // if the guess was smaller than secret_number, say "Too small!"
        Ordering::Greater => println!("Too big!"), // if the guess was bigger than secret_number, say "Too big~"
        Ordering::Equal => println!("You win!"), // if the guess was equal to the secret_number, say "You win!"
    }
}