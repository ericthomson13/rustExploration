// std library contains a definition for IpAddr but because it's not imported it doesn't conflict

enum IpAddr {
  V4(u8, u8, u8, u8),
  V6(String),
}

enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}
impl Message {
  fn call(&self) {

  }
}
struct QuitMessage; // unit struct
struct MoveMessage {
  x: i32,
  y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32); // tuple struct

fn main() {
  let home = IpAddr::V4(127, 0, 0, 1);

  let loopback = IpAddr::V6(String::from("::1"));

  let m = Message::Write(String::from("hello"));
  m.call();
}

fn route (ip_kind: IpAddr) {}