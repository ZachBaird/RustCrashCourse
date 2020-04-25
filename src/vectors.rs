// Vectors - can be thought of as sizeable arrays

use std::mem;

pub fn run() {
  // This is a vector who's values we can't change since it isn't mutable
  // We also can't push values onto the vector either
  let numbers: Vec<i32> = vec![1, 2, 3, 4];

  let mut mutable_vector: Vec<i32> = vec![1, 2, 3];

  // Attempts to add to vectors
  mutable_vector.push(5);
  mutable_vector.push(6);

  // We can remove from vectors as well
  mutable_vector.pop();

  println!("Numbers Vector: {:?}", numbers);
  println!("Mutable Numbers Vector: {:?}", mutable_vector);

  // We can change the mutable vector's existing values
  mutable_vector[1] = 10;

  // Getting a single value
  println!("Vector 2nd Pos: {}", mutable_vector[1]);

  // Getting vector length
  println!("Vector Length: {}", mutable_vector.len());

  // Vectors are stack allocated
  println!(
    "Unmutable Vector occupies {} bytes",
    mem::size_of_val(&numbers)
  );

  // Get the slice of a vector
  let slice: &[i32] = &numbers[1..3];
  println!("Slice: {:?}", slice);

  // Loop through vector values
  for x in mutable_vector.iter() {
    println!("{}", x);
  }

  // We can loop & mutate values as well
  for x in mutable_vector.iter_mut() {
    *x *= 2;
  }

  println!("Mutable Vec mutated: {:?}", mutable_vector);
}
