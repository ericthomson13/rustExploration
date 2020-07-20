fn main() {
  let s1 = String::from("hello");

  // call with argument &<var> to create reference
  let length = calculate_length(&s1);

  println!("The length of {} is {}.", s1, length);

  let mut s2 = String::from("new world");

  change(& mut s2);

  println!("{}", s2);

  // below attempts to return a Dangling Pointer and will error
  // dangle();
}

// Parameter &String to create reference and not ownership
fn calculate_length (s: &String) -> usize {
  // don't follow this line with ; it will not return the value
  s.len()
}

// Uses reference but because mutable can change the referenced var
fn change (some_string: &mut String) {
  some_string.push_str(" order");
}

// Below will error because attempts to create a dangling pointer
fn dangle () -> &String {
  let s = String::from("hello!");

  &s
}