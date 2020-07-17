// Prelude import I/O package from Standard Library
use std::io;

fn main() {
  println!("Guess a Number!");

  println!("Please input your guess.");

  // Declares guess as a mutable empty string value
  let mut guess = String::new();

  io::stdin()
    // & indicates that this is a reference and allows for other parts of program to access
    .read_line(&mut guess)
    .expect("Failed to read line");

  println!("You guessed: {}", guess);
}
