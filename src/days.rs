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

fn day3(input: &Vec<String>) -> Result<(String, String), String> {
  const BIT_WIDTH: usize = 12;

  fn bits_to_num(bits: &String) -> u32 { u32::from_str_radix(bits.as_str(), 2).unwrap() }

  let chars = input.into_iter().map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();

  let mut gamma_rate: Vec<char> = Vec::new();
  let mut epsilon_rate: Vec<char> = Vec::new();

  fn bit_counts(collection: &Vec<Vec<char>>, index: usize) -> (u32, u32) {
    let n_zeros = collection.iter().fold(0, |acc, cs| acc + if cs[index] == '0' { 1 } else { 0 });
    return (n_zeros, (collection.len() as u32) - n_zeros);
  }

  for i in 0..BIT_WIDTH {
    let (n_zeros, n_ones) = bit_counts(&chars, i);

    gamma_rate.push(if n_zeros > n_ones { '0' } else { '1' });
    epsilon_rate.push(if n_zeros < n_ones { '0' } else { '1' });
  }

  let gamma_str = gamma_rate.into_iter().collect::<String>();
  let epsilon_str = epsilon_rate.into_iter().collect::<String>();
  let power_consumption = bits_to_num(&gamma_str) * bits_to_num(&epsilon_str);

  fn filter(chars1: &Vec<Vec<char>>, chars2: &Vec<Vec<char>>, index: usize) -> (Vec<char>, Vec<char>) {
    if (chars1.len() == 1 && chars2.len() == 1) || index >= BIT_WIDTH {
      return (chars1[0].clone(), chars2[0].clone());
    }

    fn get_filtered(collection: &Vec<Vec<char>>, index: usize, predicate: impl Fn(char) -> bool) -> Vec<Vec<char>> {
      return collection.iter().filter(|cs| predicate(cs[index])).cloned().collect::<Vec<Vec<char>>>();
    }

    let (rr1, rr2): (&Vec<Vec<char>>, &Vec<Vec<char>>);
    let (r1, r2): (Vec<Vec<char>>, Vec<Vec<char>>);

    if chars1.len() == 1 {
      rr1 = chars1;
    } else {
      let (n_zeros, n_ones) = bit_counts(&chars1, index);
      let most_common = if n_zeros > n_ones { '0' } else { '1' };
      r1 = get_filtered(&chars1, index, |c| c == most_common);
      rr1 = &r1;
    }

    if chars2.len() == 1 {
      rr2 = chars2;
    } else {
      let (n_zeros, n_ones) = bit_counts(&chars2, index);
      let least_common = if n_ones < n_zeros { '1' } else { '0' };
      r2 = get_filtered(&chars2, index, |c| c == least_common);
      rr2 = &r2;
    }

    return filter(rr1, rr2, index + 1);
  }

  let (oxygen_generator_rating, co2_scrubber_rating) = filter(&chars, &chars, 0);
  let oxygen_generator_string = oxygen_generator_rating.into_iter().collect::<String>();
  let co2_scrubber_string = co2_scrubber_rating.into_iter().collect::<String>();
  let life_support_rating = bits_to_num(&oxygen_generator_string) * bits_to_num(&co2_scrubber_string);

  return Ok((power_consumption.to_string(), life_support_rating.to_string()));
}

fn day4(input: &Vec<String>) -> Result<(String, String), String> {
  struct Board {
    grid: Vec<Vec<(u8, bool)>>,
    won: bool
  }

  fn to_cell(s: &str) -> (u8, bool) { (s.trim().parse::<u8>().unwrap(), false) }

  fn find_index(grid: &Vec<Vec<(u8, bool)>>, value: (u8, bool)) -> Option<(usize, usize)> {
    for i in 0..5 {
      for j in 0..5 {
        if grid[i][j] == value {
          return Some((i, j));
        }
      }
    }

    return None;
  }

  fn board_score(grid: &Vec<Vec<(u8, bool)>>) -> u16 {
    let mut sum = 0;

    for x in 0..5 {
      for y in 0..5 {
        if !(&grid[x][y].1) {
          sum += &(grid[x][y].0 as u16);
        }
      }
    }

    return sum;
  }

  fn win_with_score(grid: &Vec<Vec<(u8, bool)>>, (i, j): (usize, usize)) -> Option<u16> {
    let complete_row = {
      let mut found = true;
      for x in 0..5 {
        if !(&grid[x][j].1) {
          found = false;
          break;
        }
      }

      found
    };

    let complete_col = {
      let mut found = true;
      for y in 0..5 {
        if !(&grid[i][y].1) {
          found = false;
          break;
        }
      }

      found
    };

    if complete_row || complete_col {
      return Some(board_score(grid));
    } else {
      return None;
    }
  }

  let numbers = input[0].split(',').map(|s| to_cell(s)).collect::<Vec<_>>();

  let mut boards = (&input[2..])
    .split(|line| line.trim().is_empty())
    .map(|grid| {
      grid
        .iter()
        .map(|line| line.trim().replace("  ", " ").split(' ').map(|s| to_cell(s)).collect::<Vec<_>>())
        .collect::<Vec<_>>()
    })
    .map(|g| Board { grid: g, won: false })
    .collect::<Vec<_>>();

  let mut first_score = None;
  let mut last_score = 0;

  for n in numbers {
    for board in boards.iter_mut() {
      if let Some((i, j)) = find_index(&board.grid, n) {
        if !board.grid[i][j].1 && !board.won {
          board.grid[i][j].1 = true;

          if let Some(score) = win_with_score(&board.grid, (i, j)) {
            board.won = true;

            if let None = first_score {
              first_score = Some((n.0 as u16) * score)
            };

            last_score = (n.0 as u16) * score;
          }
        }
      }
    }
  }

  return Ok((first_score.unwrap().to_string(), last_score.to_string()));
}

fn day5(_input: &Vec<String>) -> Result<(String, String), String> { return Err(String::from("Not implemented.")); }

fn day6(_input: &Vec<String>) -> Result<(String, String), String> { return Err(String::from("Not implemented.")); }

fn day7(_input: &Vec<String>) -> Result<(String, String), String> { return Err(String::from("Not implemented.")); }

fn day8(_input: &Vec<String>) -> Result<(String, String), String> { return Err(String::from("Not implemented.")); }

fn day9(_input: &Vec<String>) -> Result<(String, String), String> { return Err(String::from("Not implemented.")); }

fn day10(_input: &Vec<String>) -> Result<(String, String), String> { return Err(String::from("Not implemented.")); }

fn day11(_input: &Vec<String>) -> Result<(String, String), String> { return Err(String::from("Not implemented.")); }

fn day12(_input: &Vec<String>) -> Result<(String, String), String> { return Err(String::from("Not implemented.")); }

fn day13(_input: &Vec<String>) -> Result<(String, String), String> { return Err(String::from("Not implemented.")); }

fn day14(_input: &Vec<String>) -> Result<(String, String), String> { return Err(String::from("Not implemented.")); }

fn day15(_input: &Vec<String>) -> Result<(String, String), String> { return Err(String::from("Not implemented.")); }

fn day16(_input: &Vec<String>) -> Result<(String, String), String> { return Err(String::from("Not implemented.")); }

fn day17(_input: &Vec<String>) -> Result<(String, String), String> { return Err(String::from("Not implemented.")); }

fn day18(_input: &Vec<String>) -> Result<(String, String), String> { return Err(String::from("Not implemented.")); }

fn day19(_input: &Vec<String>) -> Result<(String, String), String> { return Err(String::from("Not implemented.")); }

fn day20(_input: &Vec<String>) -> Result<(String, String), String> { return Err(String::from("Not implemented.")); }

fn day21(_input: &Vec<String>) -> Result<(String, String), String> { return Err(String::from("Not implemented.")); }

fn day22(_input: &Vec<String>) -> Result<(String, String), String> { return Err(String::from("Not implemented.")); }

fn day23(_input: &Vec<String>) -> Result<(String, String), String> { return Err(String::from("Not implemented.")); }

fn day24(_input: &Vec<String>) -> Result<(String, String), String> { return Err(String::from("Not implemented.")); }

fn day25(_input: &Vec<String>) -> Result<(String, String), String> { return Err(String::from("Not implemented.")); }

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
