use std::env::{args, Args};

fn main() {
  println!("Hello, world!");
  let args: Args= args();
  println!("{:?}",args);
    
}
