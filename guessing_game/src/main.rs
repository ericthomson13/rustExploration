// Prelude
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
  println!("Guess a Number!");

  // generates random number between 1 and 100
  let secret_number = rand::thread_rng().gen_range(1, 101);

  println!("The secret number is: {}", secret_number);

  // Starts infinite loop
  loop {
    println!("Please input your guess.");

    // Declares guess as a mutable empty string value
    let mut guess = String::new();

    io::stdin()
      // & indicates that this is a reference and allows for other parts of program to access
      .read_line(&mut guess)
      .expect("Failed to read line");

    let guess: u32 = match guess.trim().parse() {
      // parse returns a Result type that is an Enum, either Ok or Err
      Ok(num) => num,
      // _ is a catchall val that will match all Err vals
      // This will ignore non-number inputs and ask for another guess
      Err(_) => continue,
    };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
      Ordering::Less => println!("Too small!"),
      Ordering::Greater => println!("Too big!"),
      Ordering::Equal => {
        println!("You win!!");
        break;
      },
    }
  }
}

// p186 in Error Handling

pub struct Guess {
  value: i32,
}

impl Guess {
  pub fn new(value: i32) -> Guess {
    if value < 1 || value > 100 {
      panic!("Guess value must be between 1 and 100, got {}", value);
    }
    Guess { value }
  }

  pub fn value (&self) -> i32 {
    self.value
  }
}