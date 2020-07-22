#[cfg(test)]

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

fn serve_order() {}

mod back_of_house {
  // structures default to private
  pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
  }

  impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(toast),
        seasonal_fruit: String::from("peaches"),
      }
    }
  }

  pub enum Appetizer {
    Soup,
    Salad,
  }

  fn fix_incorrect_order() {
    cook_order();
    super::serve_order();
  }

  fn cook_order() {}
}

// brings hosting into scope via abs path
// use crate::front_of_house::hosting

// brings hosting into scope via relative path
// use self::front_of_house::hosting


mod front_of_house;

// brings just the add_to_waitlist fn into scope
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant () {
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();

  // Order a breakfast in the summer with Rye toast
  let mut meal = back_of_house::Breakfast::summer("Rye");

  // Change our mind about what bread we'd like
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);

  let order1 = back_of_house::Appetizer::Soup;
  let order2 = back_of_house::Appetizer::Salad;
}
