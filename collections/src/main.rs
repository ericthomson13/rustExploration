fn main() {
  let mut v: Vec<i32> = Vec::new();
  // vectors can be updated
  v.push(5);
  v.push(6);
  v.push(7);
  v.push(9);

  let macroV = vec![1, 2, 3, 4, 5]; // infers that type is Vec<i32>

  // Vectors can be accessed via index
  // Vectors are 0 indexed
  let third: &i32 = &macroV[2];
  println!("the third elem in macroV is {}", third);

  // Vectors can be accessed via get() method
  match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
  }

  let mut v2 = vec![1, 2, 3, 4, 5];
  let first = &v2[0];

  // v2.push(6);

  println!("The first element is: {}", first);

  // below line panic because vector isn't that long
  // let does_not_exist = &v2[100];

  // below line doesn't panic because will return None
  let does_not_exist_2 = v2.get(100);


  let v3 = vec![100, 32, 57];
  println!("For loop through v3 via indices, i in &v");
  for i in &v {
    println!("{}", i);
  }

  let mut mutV = vec![100, 32, 57];
  for i in &mut mutV {
    *i += 50;
  }
}

fn spread_sheet () {
  enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
  }

  // putting an enum into a vector
  let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Float(10.12),
    SpreadsheetCell::Text(String::from("blue")),
  ];
}

fn strings () {
  // all strings are UTF-8 encoded
  let data = "intial contents";

  // below all create the same string
  let s1 = data.to_string();
  let s2 = "initial contents".to_string();
  let s3 = String::from(data);

  let mut s = String::from("foo");
  s.push_str(" bar");

  let mut string1 = String::from("foo");
  let string2 = "bar";
  // push_str() doesn't take ownership of string2 just uses a slice
  string1.push_str(string2);
  println!("string2 is {}", string2);

  // allows for a single character parameter using push()
  let mut pushstring = String::from("lo");
  pushstring.push('l');

  let hello = String::from("hello");
  let world = String::from("world!");
  let hello_world = hello + &world; // hello ownership is moved

  let tic = String::from("tic");
  let tac = String::from("tac");
  let toe = String::from("toe");

  // format macro makes it easier to see complex string combining
  let formatted = format!("{}-{}-{}", tic, tac, toe);
  println!("formatted is {}", formatted);

  let hola = String::from("hola");
  // below doesn't work because strings aren't indexed
  // let h = hola[0];
  // below can be used but is unpredictable because is bytes in storage
  let h2 = &hola[0..4];


  // Iterate through strings to access elements
  for c in hola.chars() {
    println!("{}", c);
  }

  for b in hola.bytes() {
    println!("{}", b);
  }
}

fn hash_maps () {

}