// Primitive Strings = Immutable, fixed-length string somewhere in memory
//                   - Cannot be mutable even with `mut` keyword
// String = Growable, heap-allocated data structure
//        - Use when you need to modify or own string data
//        - Interestingly, you still need to use the `mut` keyword to make em mutable

pub fn run() {
  // Primitive string
  let hi = "hi";

  // String
  //  - However, to still make thi
  let mut hello = String::from("Hello");

  // Getting the length of the strings
  println!("Length of hi: {}", hi.len());
  println!("Length of hello: {}", hello.len());

  // Pushes a char onto the string
  hello.push('W');
  println!("{}", hello);

  // Pushes a string onto the string
  hello.push_str("orld");
  println!("{}", hello);

  // Get the capacity of the string in bytes
  println!("Capacity: {}", hello.capacity());

  // Determine if the string is empty
  println!("Is Empty: {}", hello.is_empty());

  // Determine if the string contains "World"
  println!("Contains 'World': {}", hello.contains("World"));

  // Replace method
  println!("Replaced: {}", hello.replace("World", "There"));
  println!(
    "Contains 'World': {}",
    hello.replace("World", "There").contains("World")
  );

  // Explicit primitive string declaration
  let this_is_a_sentence: &str = "This is a sentence!";

  // Loop through the string by whitespace to get each word
  for word in this_is_a_sentence.split_whitespace() {
    println!("{}", word);
  }

  // Create a string with a certain capacity
  let mut s = String::with_capacity(10);
  s.push('a');
  s.push('b');
  //s.push('c'); // Added for assertion test

  println!("Capacity of {}: {} bytes", s, s.capacity());

  // Assertion testing
  // - Will stop the program if the assertion fails. Inline testing??
  assert_eq!(2, s.len());
  assert_eq!(10, s.capacity());
}
