// Prelude import I/O package from Standard Library
use std::io;
use rand::Rng;

fn main() {
  println!("Guess a Number!");

  // generates random number between 1 and 100
  let secret_number = rand::thread_rng().gen_range(1, 101);

  println!("The secret number is: {}", secret_number);

  println!("Please input your guess.");

  // Declares guess as a mutable empty string value
  let mut guess = String::new();

  io::stdin()
    // & indicates that this is a reference and allows for other parts of program to access
    .read_line(&mut guess)
    .expect("Failed to read line");

  println!("You guessed: {}", guess);
}
