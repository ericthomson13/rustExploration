fn main() {
  let word = String::from("hello world");

  let space = first_word(&word);

  println!("Space is after {}", space);
}


fn first_word (s: &String) -> &str {
  // converts string to array of bytes
  let bytes = s.as_bytes();

  // iter() returns each element
  // enumerate() returns tuple of [index, reference]
  for (i, &item) in bytes.iter().enumerate() {
    // if finds a space returns the index
    if item == b' ' {
      return &s[0..i];
    }
  }

  // if no space in input returns string length
  &s[..]
}