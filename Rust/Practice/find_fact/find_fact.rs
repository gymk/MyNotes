use std::env;

fn fact(num:u64) -> u64 {
  if num <=1 {
    return 1;
  }
  num * fact(num - 1)
}

fn main() {
  let args: Vec<String> = env::args().collect();
  println!("{:?}", fact(args[1].parse::<u64>().unwrap()));
}