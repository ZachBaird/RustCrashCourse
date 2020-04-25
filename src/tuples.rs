// Tuples groupe together values of different types
// They have a maximum of 12 elements

pub fn run() {
  let person: (&str, &str, i8) = ("Zach", "New York", 25);

  println!("{} is from {} and is {}", person.0, person.1, person.2);
}
