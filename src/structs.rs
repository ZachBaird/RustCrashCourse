// Structs - Used to create custom data types

// Traditional Struct
struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

// Tuple Struct
struct TupleColor(u8, u8, u8);

// Person Struct
struct Person {
  first_name: String,
  last_name: String,
}

impl Person {
  // Construct person
  fn new(first: &str, last: &str) -> Person {
    Person {
      first_name: first.to_string(),
      last_name: last.to_string(),
    }
  }

  fn full_name(&self) -> String {
    format!("{} {}", self.first_name, self.last_name)
  }

  fn set_last_name(&mut self, last: &str) {
    self.last_name = last.to_string()
  }
}

pub fn run() {
  let c = Color {
    red: 255,
    green: 0,
    blue: 0,
  };

  println!("Color: {} {} {}", c.red, c.green, c.blue);

  let tc = TupleColor(255, 0, 0);
  println!("Tuple Color: {} {} {}", tc.0, tc.1, tc.2);

  let p = Person::new("Zach", "Baird");
  println!("{}", p.full_name());

  p.set_last_name("Doe");
  println!("{}", p.full_name());
}
