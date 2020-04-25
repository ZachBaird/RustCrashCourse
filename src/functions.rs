// Functions

pub fn run() {
  greeting("Feasgar math", "Zach");
  println!("Sum: {}", add(5, 4));

  let sum = add(5, 5);

  // Closure
  let add_nums = |n1: i32, n2: i32| n1 + n2;
  println!("Closure Sum: {}", add_nums(3, sum));
}

fn greeting(greet: &str, name: &str) {
  println!("{}, {}!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
  n1 + n2
}
