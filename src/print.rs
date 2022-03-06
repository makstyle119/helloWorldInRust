pub fn run() {
  // pub is use for public function, fn is use for function and run is the name of the function

  println!("Hello From print.rs"); // println! is use to print

  println!("Number: {}", 1); // this is use to print an int

  // Basic formating
  println!("{} Is from {}", "MAK", "PAKISTAN"); // {} is replace by the next value

  // Positional Argument
  println!(
    "{0} Is from {1} and {0} like {2}",
    "MAK", "PAKISTAN", "CODE"
  ); // {} is replace by the index value

  // Name Argument
  println!("{name} like to play {game}", name = "MAK", game = "chess"); // {} is replace by the key value

  // Placeholder traits
  println!("Binary: {:b} Hex: {:x}, Octal: {:o}", 10, 10, 10);

  // Placeholder for debug traits
  println!("{:?}", (12, true, "String"));

  // Basic Math
  println!("10 + 10 = {}", 10 + 10);
}
