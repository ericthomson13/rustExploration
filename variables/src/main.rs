fn main() {

  // Mutable Variables

  // if !mut then will throw compile error
  // variables are immutable by default
  let mut x = 5;

  println!("The valie of x is: {}", x);
  x = x + 1;
  println!("The value of x is: {}", x);
  x = x * 2;
  println!("The value of x is: {}", x);

  // Constants

  // const is ALWAYS immutable, not just by default
  // _ can be used to improve readability
  const MAX_POINTS: u32 = 100_000;

  println!("The MAX-POINTS value is ALWAYS {}", MAX_POINTS);

  // Can reuse variable name with different type if use let keyword again
  let spaces = "     ";
  let spaces = spaces.len();

  println!("There were {} spaces", spaces);

  // Immutability of type w/o let keyword reuse

  // Can't make spaces a mutable variable and change the type
  // Below 2 lines will not compile
  // let mut spaces = "        ";
  // spaces = spaces.len();

  println!("There were {} spaces", spaces);


  // Errors when above int bounds

  // Below 2 lines "overflow" is above u8 integer and will cause error at compile unless --release where changes to "wraparound" 256 = 0, 257 = 1
  // let overflow: u8 = 277;
  // println!("This overflow error {}", overflow);


  // Floating Points

  let z = 2.0; // defaults to f64
  let y: f32 = 3.0; // declared f32

  println!("{} is a f64 variable and {} is a f32 variable", z, y);

  // Booleans

  let t = true;
  let f: bool = false;

  println!("t is {} and f is {}", t, f);
}
