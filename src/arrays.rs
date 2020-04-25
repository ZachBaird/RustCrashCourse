// Arrays - Fixed lists where elements are the same data types

use std::mem;

pub fn run() {
  let numbers: [i32; 5] = [1, 2, 3, 4, 5];
  let mut mutable_numbers: [i32; 4] = [2, 4, 6, 8];

  println!("{:?}", numbers);

  // Get a single value from the array
  println!("Single Value: {}", numbers[0]);

  // We can mutate positions in an array
  println!("Entire Mutable Array: {:?}", mutable_numbers);
  mutable_numbers[2] = 1;
  println!("Changed Mutable Array: {:?}", mutable_numbers);

  // Arrays are stack allocated. We can determine how much
  //  space an array is taking up with std::mem
  println!(
    "Non-mutable Array occupies {} bytes",
    mem::size_of_val(&numbers)
  );
  println!(
    "Mutable Array occupies {} bytes",
    mem::size_of_val(&mutable_numbers)
  );

  // We can grab slices of an array
  let slice: &[i32] = &numbers[0..2];
  println!("Slice: {:?}", slice);
}
