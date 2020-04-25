/*
  Primitive Types:
    Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128
              (these are different because they allocate the
              number of bits of memory they take in)
              ( u = unsigned: no negative values )
              ( i = integers: can be pos and neg )
    Floats: f32, f64
    Boolean (bool)
    Characters (char)
    Tuples
    Arrays - fixed length
*/

// Rust is a statically typed language, which mens that it must know
//  the types of all variables at compile time, however, the compiler
//  can usually infer what type we want to use based on the value and
//  how we use it.

pub fn run() {
  // An example of type inference
  //  Defaults to i32
  let x = 1;

  // Defaults to f64
  let y = 2.5;

  // Examples of explicit types
  let z: i64 = 4545454545454545;

  // Find max size of i32 and i64 data types
  println!("Max of i32 type: {}", std::i32::MAX);
  println!("Max of i64 type: {}", std::i64::MAX);

  // Boolean
  let is_active = true;
  let is_explicitly_active: bool = true;

  let is_greater = 10 > 5;

  println!("{:?}", (x, y, z, is_active, is_explicitly_active));
  println!("Is 10 > 5: {:?}", (is_greater));

  // Char
  let a1 = 'a';
  let b1: char = 'b';

  // Since chars can be any unicode character,
  //  we can bring in unicode emojis
  let face = '\u{1F600}';

  println!("{:?}", (a1, b1, face));
}
