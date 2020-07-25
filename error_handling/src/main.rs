
/*

Most of the fn calls in this file will have to be commented in/out
to run each example as several call unrecoverable errors

*/

use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {

  // UNRECOVERABLE ERROR INTRO

  // our code calling a panic!
  // crash_and_burn();

  // panic! called from within a library
  // run with for full log: RUST_BACKTRACE=full cargo run
  // bug_call_panic();

  // RECOVERABLE ERROR INTRO

  // panic! called if doesn't find via .unwrap()
  // unwrap_example();

  // logs custom error message if doesn't find file
  // expect_example();

  // below attempts to create file if doesn't exist
  // open_file();

  // below attempts to read a string from hello.txt
  read_username_from_file();

}

fn crash_and_burn () {
  panic!("crash and burn");
}

fn bug_call_panic () {
  let v = vec![1, 2, 3];
  // passing an invalid index calls panic from within vector lib
  // this would cause a buffer overread (and potential security issue) without this error
  v[99];
}

// enum Result<T, E> {
//   Ok(T),
//   Err(E),
// }

fn open_file () {
  // returns instance of above enum
  let f = File::open("hello.txt");

  let f = match f {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create("hello.txt") {
        Ok(fc) => fc,
        Err(e) => panic!("Problem creating the file: {:?}", e),
      },
      other_error => {
        panic!("Problem opening the file: {:?}", other_error)
      }
    },
  };
  // easier to read version
  // let f = File::open("hello.txt").unwrap_or_else(|error| {
  //   if error.kind() == ErrorKind::NotFound {
  //     File::create("hello.txt").unwrap_or_else(|error| {
  //       panic!("Problem creating the file {:?}", error);
  //     })
  //   } else {
  //     panic!("Problem opening the file {:?}", error);
  //   }
  // });
}

fn unwrap_example () {
  // calls panic! if Err otherwise returns Ok
  let f = File::open("hello.txt").unwrap();
}

fn expect_example () {
  // can make the panic! call easier to track down with custom message
  let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

fn read_username_from_file () -> Result<String, io::Error> {
  let mut s = String::new();

  // ? operator helps eliminate a lot of boilerplate code
  // ? can only be used with return val of Result or Option
  File::open("hello.txt")?.read_to_string(&mut s)?;

  Ok(s)
}