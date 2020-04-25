// Reference pointers - point to a resource in memory
pub fn run() {
  // Primitive Array
  let arr1 = [1, 2, 3];
  let arr2 = arr1;

  println!("Values: {:?}", (arr1, arr2));

  // With non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value. You'll need to use a reference (&) to point to the resource

  // Vectors
  let vector1: Vec<i32> = vec![1, 2, 3];
  let vector2 = &vector1;

  println!("Vector Values: {:?}", (&vector1, vector2));
}
