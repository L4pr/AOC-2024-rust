use std::collections::{HashMap, HashSet, VecDeque};
use advent_of_code::get_lines;

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = get_lines(input);
    let mut result = 0;

    for line in lines {
        let mut number: u64 = line.parse().unwrap();
        for _ in 0..2000 {
            number = calculate_secret_number(number);
        }
        result += number;
    }

    Some(result)
}

fn calculate_secret_number(secret_number: u64) -> u64 {
    let mut result = mix_and_prune(secret_number, secret_number << 6);
    result = mix_and_prune(result, result >> 5);
    mix_and_prune(result, result << 11)
}

fn mix_and_prune(secret_number: u64, number: u64) -> u64 {
    (secret_number ^ number) % 16777216
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = get_lines(input);

    let mut sequences: HashMap<VecDeque<i32>, u64> = HashMap::new();

    for line in lines {
        let mut number: u64 = line.parse().unwrap();
        let mut changes_temp = VecDeque::new();
        let mut price = number % 10;

        let mut sequences_been: HashSet<VecDeque<i32>> = HashSet::new();

        for _ in 0..2000 {
            number = calculate_secret_number(number);
            let new_price = number % 10;
            changes_temp.push_back(new_price as i32 - price as i32);
            if changes_temp.len() > 4 {
                changes_temp.pop_front();
                if !sequences_been.contains(&changes_temp) {
                    let entry = sequences.entry(changes_temp.clone()).or_insert(0);
                    *entry += new_price;
                    sequences_been.insert(changes_temp.clone());
                }
            }
            price = new_price;
        }
    }

    println!("{}", sequences.get(&VecDeque::from([-2,1,-1,3])).unwrap());

    Some(*sequences.values().max().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(37327623));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
