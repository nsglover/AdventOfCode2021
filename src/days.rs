// Imports for input manager
use std::fs::File;
use std::io::BufReader;
use std::io::Lines;

fn day1(_input: &Vec<String>) -> Result<(String, String), String> {
  return Err(String::from("Not implemented."));
}

fn day2(_input: &Vec<String>) -> Result<(String, String), String> {
  return Err(String::from("Not implemented."));
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
    _ => Err(format!("Day {} is not a valid day.", day)),
  }
}
