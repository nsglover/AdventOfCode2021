mod days;

use std::env;
use std::fs::File;
use std::io::{stdout, BufRead, BufReader, Lines, Write};

fn get_input_reader(day: u8) -> Option<Lines<BufReader<File>>> {
  let f = File::open(format!("puzzle-inputs/day{}.txt", day));
  match f {
    Ok(file) => {
      return Some(BufReader::new(file).lines());
    }
    Err(_) => {
      println!("Unable to load file: puzzle-inputs/day{}.txt", day);
      return None;
    }
  }
}

fn solve_day(day: u8) {
  println!("Solving day {}...\n", day);

  let r = get_input_reader(day);
  match r {
    Some(reader) => {
      let result = days::solve_day(day, reader);

      match result {
        Ok((part1, part2)) => {
          let stdout = stdout();
          let mut out = stdout.lock();

          println!("Part 1: \n");
          let _ = out.write_all(part1.as_bytes());

          println!("\n\nPart 2: \n");
          let _ = out.write_all(part2.as_bytes());

          println!();
        }
        Err(err) => println!("Day {} encountered an error: {}", day, err)
      }
    }
    None => return
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    println!("Solving all days. Run advent_of_code_2021 n to solve a specific day n instead.");
    for day in 1..=25 {
      println!("\n----------------------------------------------------------\n");
      solve_day(day);
    }

    println!("\n----------------------------------------------------------");
  } else {
    let request = args[1].parse::<u8>();

    match request {
      Ok(day) => {
        if (1..=25).contains(&day) {
          solve_day(day)
        } else {
          println!("Day {} is not a valid day.", day)
        }
      }
      Err(_) => println!("Invalid input.")
    }
  }
}
