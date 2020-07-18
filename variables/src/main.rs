fn main() {
  // if !mut then will throw compile error
  // variables are immutable by default
  let mut x = 5;

  println!("The valie of x is: {}", x);
  x = x + 1;
  println!("The value of x is: {}", x);
  x = x * 2;
  println!("The value of x is: {}", x);

  // const is ALWAYS immutable, not just by default
  // _ can be used to improve readability
  const MAX_POINTS: u32 = 100_000;

  println!("The MAX-POINTS value is ALWAYS {}", MAX_POINTS);

  // Can reuse variable name with different type if use let keyword again
  let spaces = "     ";
  let spaces = spaces.len();

  println!("There were {} spaces", spaces);

  // Can't make spaces a mutable variable and change the type
  // Below 2 lines will not compile
  // let mut spaces = "        ";
  // spaces = spaces.len();

  println!("There were {} spaces", spaces);
}
