use std::io;
use std::io::Read;
use std::collections::HashMap;

fn parse(input: &String) -> (Vec<i32>, Vec<i32>) {
  let mut list1: Vec<i32> = Vec::new();  
  let mut list2: Vec<i32> = Vec::new();  

  for line in input.lines() {
    let nums: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
    list1.push(nums[0]);
    list2.push(nums[1]);
  }

  (list1, list2)
}

fn part1(data: &(Vec<i32>, Vec<i32>)) -> i32 {
  let (list1, list2) = data;

  let mut sorted_list1 = list1.clone();
  let mut sorted_list2 = list2.clone();

  sorted_list1.sort();
  sorted_list2.sort();

  let n = list1.len();
  let mut total_dist = 0;

  for i in 0..n {
    total_dist += (sorted_list1[i] - sorted_list2[i]).abs();
  }

  total_dist
}

fn part2(data: &(Vec<i32>, Vec<i32>)) -> i32 {
  let (list1, list2) = data;

  let frequencies = list2.iter().fold(HashMap::new(), |mut map, &val| {
    *map.entry(val).or_insert(0) += 1;
    map
  });

  let mut similarity_score = 0;

  for &x in list1.iter() {
    similarity_score += x * frequencies.get(&x).copied().unwrap_or(0);
  }

  similarity_score
}

fn main() -> io::Result<()> {
  let mut input = String::new();
  io::stdin().read_to_string(&mut input)?;

  let data = parse(&input);
  println!("Part 1: {}", part1(&data));
  println!("Part 2: {}", part2(&data));

  Ok(())
}

