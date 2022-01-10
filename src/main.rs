mod print; // mod is use to import other file, (btw ; is required in rust)

fn main() {
    // println!("Hello, world!");
    print::run(); // call run function from print.rs file
}
