use std::io; // imports the io lib from std, allows for strings and stuff

fn main() { // main function of the program, all the code goes into here to get run
    println!("Guess the number!"); // prints out a single line, "guess the number!" using a rust macro

    println!("Please input your guess."); // same as last

    let mut guess = String::new();
    // sets a variable, guess. normally, variables are `immutable` which means it cannot change. 
    // but if you specify `mut`, it will be mutable, i can change it after inputting it. 
    // the `String::New()` just creates a new empty string that we can change later.
    io::stdin() // from the io module, we'll use stdin. stdin allows for user input
        .read_line(&mut guess) 
        // .read_line handles user input, from the keyboard.
        // we pass through `&mut guess` to store the user input. 
        // this will store it in the previously defined `guess` variable. we add the &mut so it knows it's mutable.
        .expect("Failed to read line"); // error handling. if it fails to read, it'll output this.

    println!("You guessed: {}", guess); 
    // this is the last line of code. this outputs "You guessed: {guess}"
    // but how? the {} indicates that the second param, guess, should be placed here.
    // resulting in "You guessed: {number you guessed}"
}