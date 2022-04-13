// Imports for input manager
use std::fs::File;
use std::io::BufReader;
use std::io::Lines;

// Imports for parsing and text processing
use regex::Regex;

// Imports for iteration
use std::iter;

fn day1(input: &Vec<String>) -> Result<(String, String), String> {
  fn sum_increases(list: &Vec<u32>) -> u32 {
    let iter_offset = iter::once(&std::u32::MAX).chain(list.iter());
    let pairs = list.iter().zip(iter_offset);
    let diffs = pairs.map(|(d, d_prev)| if d > d_prev { 1 } else { 0 });
    diffs.fold(0, |acc, k| acc + k)
  }

  let depths: Vec<u32> = input.iter().map(|s| s.parse::<u32>().unwrap()).collect();
  let shift_0_iter = depths.iter();
  let mut shift_1_iter = shift_0_iter.clone();
  let _ = shift_1_iter.next();
  let mut shift_2_iter = shift_1_iter.clone();
  let _ = shift_2_iter.next();

  let triples = shift_0_iter.zip(shift_1_iter.zip(shift_2_iter));
  let three_measurement_windows = triples.map(|(d0, (d1, d2))| d0 + d1 + d2).collect();

  return Ok((sum_increases(&depths).to_string(), sum_increases(&three_measurement_windows).to_string()));
}

fn day2(input: &Vec<String>) -> Result<(String, String), String> {
  enum Direction {
    Forward,
    Down,
    Up
  }

  fn parse_cmd(cmd: &String) -> (Direction, i32) {
    let pattern = Regex::new(r"(?P<dir>\w+)\s(?P<amt>[0-9]+)").unwrap();
    let captures = pattern.captures(cmd).unwrap();
    let direction = captures.name("dir").unwrap().as_str();
    let amount = captures.name("amt").unwrap().as_str();

    let amount = amount.parse::<i32>().unwrap();
    match direction {
      "forward" => (Direction::Forward, amount),
      "down" => (Direction::Down, amount),
      "up" => (Direction::Up, amount),
      _ => (Direction::Forward, 0)
    }
  }

  let parsed = input.iter().map(|s| parse_cmd(s));

  let (h1, d1) = parsed.clone().fold((0, 0), |(h, d), (dir, amt)| match dir {
    Direction::Forward => (h + amt, d),
    Direction::Down => (h, d + amt),
    Direction::Up => (h, d - amt)
  });

  let (h2, d2, _) = parsed.fold((0, 0, 0), |(h, d, aim), (dir, amt)| match dir {
    Direction::Forward => (h + amt, d + aim * amt, aim),
    Direction::Down => (h, d, aim + amt),
    Direction::Up => (h, d, aim - amt)
  });

  return Ok(((h1 * d1).to_string(), (h2 * d2).to_string()));
}

fn day3(_input: &Vec<String>) -> Result<(String, String), String> {
  return Err(String::from("Not implemented."));
}

fn day4(_input: &Vec<String>) -> Result<(String, String), String> {
  return Err(String::from("Not implemented."));
}

fn day5(_input: &Vec<String>) -> Result<(String, String), String> {
  return Err(String::from("Not implemented."));
}

fn day6(_input: &Vec<String>) -> Result<(String, String), String> {
  return Err(String::from("Not implemented."));
}

fn day7(_input: &Vec<String>) -> Result<(String, String), String> {
  return Err(String::from("Not implemented."));
}

fn day8(_input: &Vec<String>) -> Result<(String, String), String> {
  return Err(String::from("Not implemented."));
}

fn day9(_input: &Vec<String>) -> Result<(String, String), String> {
  return Err(String::from("Not implemented."));
}

fn day10(_input: &Vec<String>) -> Result<(String, String), String> {
  return Err(String::from("Not implemented."));
}

fn day11(_input: &Vec<String>) -> Result<(String, String), String> {
  return Err(String::from("Not implemented."));
}

fn day12(_input: &Vec<String>) -> Result<(String, String), String> {
  return Err(String::from("Not implemented."));
}

fn day13(_input: &Vec<String>) -> Result<(String, String), String> {
  return Err(String::from("Not implemented."));
}

fn day14(_input: &Vec<String>) -> Result<(String, String), String> {
  return Err(String::from("Not implemented."));
}

fn day15(_input: &Vec<String>) -> Result<(String, String), String> {
  return Err(String::from("Not implemented."));
}

fn day16(_input: &Vec<String>) -> Result<(String, String), String> {
  return Err(String::from("Not implemented."));
}

fn day17(_input: &Vec<String>) -> Result<(String, String), String> {
  return Err(String::from("Not implemented."));
}

fn day18(_input: &Vec<String>) -> Result<(String, String), String> {
  return Err(String::from("Not implemented."));
}

fn day19(_input: &Vec<String>) -> Result<(String, String), String> {
  return Err(String::from("Not implemented."));
}

fn day20(_input: &Vec<String>) -> Result<(String, String), String> {
  return Err(String::from("Not implemented."));
}

fn day21(_input: &Vec<String>) -> Result<(String, String), String> {
  return Err(String::from("Not implemented."));
}

fn day22(_input: &Vec<String>) -> Result<(String, String), String> {
  return Err(String::from("Not implemented."));
}

fn day23(_input: &Vec<String>) -> Result<(String, String), String> {
  return Err(String::from("Not implemented."));
}

fn day24(_input: &Vec<String>) -> Result<(String, String), String> {
  return Err(String::from("Not implemented."));
}

fn day25(_input: &Vec<String>) -> Result<(String, String), String> {
  return Err(String::from("Not implemented."));
}

pub fn solve_day(day: u8, input: Lines<BufReader<File>>) -> Result<(String, String), String> {
  let lines = &input.map(|opt| opt.unwrap()).collect();
  match day {
    1 => day1(lines),
    2 => day2(lines),
    3 => day3(lines),
    4 => day4(lines),
    5 => day5(lines),
    6 => day6(lines),
    7 => day7(lines),
    8 => day8(lines),
    9 => day9(lines),
    10 => day10(lines),
    11 => day11(lines),
    12 => day12(lines),
    13 => day13(lines),
    14 => day14(lines),
    15 => day15(lines),
    16 => day16(lines),
    17 => day17(lines),
    18 => day18(lines),
    19 => day19(lines),
    20 => day20(lines),
    21 => day21(lines),
    22 => day22(lines),
    23 => day23(lines),
    24 => day24(lines),
    25 => day25(lines),
    _ => Err(format!("Day {} is not a valid day.", day))
  }
}
