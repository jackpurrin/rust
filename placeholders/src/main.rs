fn main() {
    let x = 5; 
    let y = 10;
    // defines two immutable variables, x and y

    println!("x = {x} and y + 2 = {}", y + 2); 
    // prints out "x = 5 and y + 2 = 12"
    // how? well the first placeholder, {x}, locates the variable x and places it in the output. 
    // the x inside of the curly brackets just defines what variable to use 
    // then the second placeholder, {}, just uses the second parameter defined in the print statement. 
    // what does y + 2 equal? 12. so it outputs 12.
}
