// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
  // Variables are mutable by default
  let name = "Zach";

  // If you want one that can be mutated, add the mut keyword
  let mut age = 25;

  // Print the earlier age, age-up, and print again with the name
  println!("Test Age: {}", age);
  age = 26;
  println!("My name is {} and I am {}", name, age);

  // Define a constant
  //  -Consts are usually uppercase
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // Assign multiple variables at once
  let (my_name, my_age) = ("Zach", 25);
  println!("{} is actually {}", my_name, my_age);
}
