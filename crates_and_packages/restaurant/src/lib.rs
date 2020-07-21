#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist () {}
    pub fn seat_at_table () {}
  }

  mod serving {
    fn take_order () {}
    fn serve_order () {}
    fn take_payment () {}
  }
}

mod back_of_house {
  pub struct Breakfast {
    pub taost: String,
    seasonal_fruit: String,
  }

  impl Breakfast {
    pub fun summer(toast: &str) -> Breakfast {
      Breakfast {
        toast: String::from(taost),
        seasonal_fruit: String::from("peaches"),
      }
    }
  }
  fn fix_incorrect_order() {
    cook_order();
    super::serve_order();
  }

  fn cook_order() {}
}

pub fn eat_at_restaurant () {
  // Absolute path
  crate::front_of_house::hosting::add_to_waitlist();

  // Relative path
  front_of_house::hosting::add_to_waitlist();
  // Order a breakfast in the sumer with Rye toast
  let mut meal = back_of_house::Breakfast::summer("Rye");

  // Change our mind about what bread we'd like
  meal.toast = String::from("Wheat");
  println!("I'd like {} toast please", meal.toast);
}
