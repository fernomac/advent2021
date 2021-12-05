use std::io;
use std::io::prelude::*;
use std::fs;

struct Point {
  x: u32,
  y: u32,
}

impl Point {
  fn new() -> Point {
    Point{ x: 0, y: 0 }
  }

  fn up(&mut self, d: u32) {
    self.y -= d
  }
 
  fn down(&mut self, d: u32) {
    self.y += d
  }

  fn forward(&mut self, d: u32) {
    self.x += d
  }
}

struct AimedPoint {
  x:   u32,
  y:   u32,
  aim: u32,
}

impl AimedPoint {
  fn new() -> AimedPoint {
    AimedPoint{ x: 0, y: 0, aim: 0 }
  }

  fn up(&mut self, d: u32) {
    self.aim -= d
  }

  fn down(&mut self, d: u32) {
    self.aim += d
  }

  fn forward(&mut self, d: u32) {
    self.x += d;
    self.y += d * self.aim
  }
}

struct Command {
  direction: String,
  distance:  u32,
}

fn parse(cmd: String) -> Command {
  let (dir, dis) = cmd.split_once(' ').unwrap();
  Command{
    direction: String::from(dir),
    distance:  dis.parse().unwrap(),
  }
}

fn main() {
  let input = io::BufReader::new(fs::File::open("input").unwrap());
  let cmds: Vec<Command> = input.lines()
    .map(|l| l.unwrap())
    .map(|l| parse(l))
    .collect();

  {
    let mut point = Point::new();

    for cmd in &cmds {
      match cmd.direction.as_str() {
        "up"      => point.up(cmd.distance),
        "down"    => point.down(cmd.distance),
        "forward" => point.forward(cmd.distance),
        _         => panic!("oh no"),
      }
    }

    println!("part one: ({}, {}) = {}", point.x, point.y, point.x * point.y)
  }

  {
    let mut point = AimedPoint::new();

    for cmd in &cmds {
      match cmd.direction.as_str() {
        "up"      => point.up(cmd.distance),
        "down"    => point.down(cmd.distance),
        "forward" => point.forward(cmd.distance),
        _         => panic!("oh no"),
      }
    }

    println!("part two: ({}, {}) = {}", point.x, point.y, point.x * point.y)
  }
}
