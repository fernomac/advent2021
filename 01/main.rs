use std::io;
use std::io::prelude::*;
use std::fs;

fn main() {
  let input = io::BufReader::new(fs::File::open("input").unwrap());
  let depths: Vec<i32> = input.lines().map(|l| l.unwrap().parse().unwrap()).collect();

  {
    let mut last = depths[0];
    let mut increases = 0;

    for i in 1..depths.len() {
      let depth = depths[i];
      if depth > last {
        increases += 1;
      }
      last = depth;
    }

    println!("part one: {} increases", increases);
  }

  {
    let mut last = depths[0] + depths[1] + depths[2];
    let mut increases = 0;

    for i in 1..depths.len()-2 {
      let sum = depths[i] + depths[i+1] + depths[i+2];
      if sum > last {
        increases += 1;
      }
      last = sum;
    }

    println!("part two: {} increases", increases);
  }
}
