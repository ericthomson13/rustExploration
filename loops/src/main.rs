fn main() {
  loop_counter();
  while_coundown(14);
  for_through_arr();
}

fn loop_counter () {
  let mut counter = 0;

  let result = loop {
    counter += 1;

    if counter == 10 {
      // without break statement will run infinite loop
      break counter * 2;
    }
  };

  println!("The result is {}", result);
}

fn while_coundown (num: i32) {
  println!("Countdown started at {}", num);
  let mut counter = num;

  while counter != 0 {
    println!("{}!", counter);
    counter -= 1;
  }

  println!("Countdown Complete!")
}

fn for_through_arr () {
  let arr = [10, 20, 30, 40, 50];

  for elem in arr.iter() {
    println!("Value from arr is {}", elem);
  }
}

fn for_liftoff () {
  // (1..4) uses a range and then .rev() to reverse, both are std lib
  for number in (1..4).rev() {
    println!("{}!", number);
  }
  println!("Liftoff!")
}