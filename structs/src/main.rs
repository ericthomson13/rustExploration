fn main() {

  let user1 = User {
    email: String::from("bob@example.com"),
    username: String::from("bobsusername123"),
    sign_in_count: 1,
    active: true,
  };

  let user2 = User {
    email: String::from("alice@example.com"),
    username: String::from("alicesusername123"),
    ..user1  // this pulls the remaining values from user1 values
  };

  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);
}

struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

fn build_user(email: String, username: String) -> User {
  User {
    email,
    username,
    active: true,
    sign_in_count: 1,
  }
}

// Tuple Structures
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

