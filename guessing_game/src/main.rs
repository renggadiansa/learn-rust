use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    
    let mut guess = String::new();

    let apples = 5; // immutable
    let bananas = 3; // immutable
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {guess}");

    let x = 5;
    let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);

}