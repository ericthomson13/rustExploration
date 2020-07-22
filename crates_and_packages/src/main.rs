// specify full path when bringing in structs enums and other items
use std::collections::HashMap;

fn main() {
  let mut map = HashMap::new();
  map.insert(1, 2);
}

// bring in two parent modules because want to use Result from both
// use std::fmt;
// use std::io;
// fn function1() -> fmt::Result {}
// fn function2() -> io::Result<()> {}

// bring in two modules with same names using AS
// use std::fmt::Result;
// use std::io::Result as IoResult;
// fn function1() -> fmt::Result {}
// fn function2() -> IoResult<()> {}