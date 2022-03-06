// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
  let name = "MAK";
  let mut age = 20;
  age = 21;

  println!("My Name Is {} and I'm {}", name, age);

  // Define Constant
  const ID: i32 = 001;
  println!("Id {}", ID);

  // Assain multiple vars
  let (my_name, my_age) = ("MAK", 20);
  println!("{} is {}", my_name, my_age);
}
