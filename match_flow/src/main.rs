#[derive(Debug)] // so we can inspect usState

enum UsState {
  Alabama,
  Alaska,
  Arizona,
  Arkansas,
  California,
  Colorado,
  Connecticut,
  Delaware,
  Florida,
  Georgia,
  Hawaii,
  Idaho,
  Illinois,
  Indiana,
  Iowa,
  Kansas,
  Kentucky,
  Louisiana,
  Maine,
  Maryland,
  Massachusetts,
  Michigan,
  Minnesota,
  Mississippi,
  Missouri,
  Montana,
  Nebraska,
  Nevada,
  NewHampshire,
  NewJersey,
  NewMexico,
  NewYork,
  NorthCarolina,
  NorthDakota,
  Ohio,
  Oklahoma,
  Oregon,
  Pennsylvania,
  RhodeIsland,
  SouthCarolina,
  SouthDakota,
  Tennessee,
  Texas,
  Utah,
  Vermont,
  Virginia,
  Washington,
  WestVirginia,
  Wisconsin,
  Wyoming,
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn main() {
  value_in_cents(Coin::Quarter(UsState::Alaska));

  let five = Some(5);
  let six = plus_one(five);
  let none = plus_one(None);

  println!("five is {:?}, six is {:?}, none is {:?}", five, six, none);

  some_val_match();

  if_let_u8(3);
}

fn value_in_cents(coin: Coin) -> u8 {

  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter(state) => {
      println!("State quarter from {:?}!", state);
      25
    },
  }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    // without the None case this will not compile
    None => None,
    Some(i) => Some(i + 1),
  }
}

fn some_val_match () {
  let some_u8_value = 0u8;
  match some_u8_value {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    // below _ will match any value
    _ => (),
  }
}

fn if_let_u8 (val: u8) {
  // if let declares var and checks equality in same line
  // if let decreasese code matching when only care about a single case
  if let Some(3) = Some(val) {
    println!("if_let_u8() matched Some(3)");
  }
}

fn if_let_else (coin: Coin) -> u64 {
  let mut count = 0;
  if let Coin::Quarter(state) = coin {
    println!("State quareter from {:?}", state);
  } else {
    count += 1;
  }
  count;
}