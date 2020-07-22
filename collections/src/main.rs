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