use num_shaman::Shaman;
use std::convert::From;

fn main() {
  println!("Pi: {:?}", Shaman::<f64>::from(std::f64::consts::PI).sin());
}
