use std::io;
use std::io::Read;

fn parse(input: &String) -> Vec<Vec<i32>> {
  input.lines().map(|line| {
    line.split_whitespace().map(|x| x.parse().unwrap()).collect()
  }).collect()
}

fn is_safe(report: &Vec<i32>) -> bool {
  if report.len() <= 1 {
    return true
  }

  let increasing: bool = report[0] < report[1];
  for i in 0..(report.len() - 1) {
    if increasing != (report[i] < report[i + 1]) {
      return false;
    }
    
    let diff = (report[i] - report[i + 1]).abs();
    if diff < 1 || diff > 3 {
      return false;
    }
  }

  true
}

fn is_safe_with_removal(report: &Vec<i32>) -> bool {
  for i in 0..report.len() {
    let mut report_slice = report.clone();
    report_slice.remove(i);
    
    if is_safe(&report_slice) {
      return true;
    }
  }

  false
}

fn part1(data: &Vec<Vec<i32>>) -> i32 {
  data.iter().filter(|report| is_safe(report)).count() as i32
}

fn part2(data: &Vec<Vec<i32>>) -> i32 {
  data.iter().filter(|report| is_safe_with_removal(report)).count() as i32
}

fn main() -> io::Result<()> {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input)?;

  let data = parse(&input);
  println!("Part 1: {}", part1(&data));
  println!("Part 2: {}", part2(&data));

  Ok(())
}
