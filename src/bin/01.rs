use advent_of_code::get_lines;
use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = get_lines(input);
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        left.push(parts[0].parse::<i32>().unwrap());
        right.push(parts[1].parse::<i32>().unwrap());
    }
    left.sort();
    right.sort();
    let mut result = 0;
    for i in 0..left.len() {
        let temp = left[i] - right[i];
        if temp < 0 {
            result -= temp;
        } else {
            result += temp;
        }
    }
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = get_lines(input);
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        left.push(parts[0].parse::<i32>().unwrap());
        right.push(parts[1].parse::<i32>().unwrap());
    }
    left.sort();
    right.sort();
    let mut result = 0;
    let mut right_index = 0;
    let mut transposition_table: HashMap<i32, i32> = HashMap::new();
    for i in 0..left.len() {
        let number = left[i];
        if transposition_table.contains_key(&number) {
            result += number * transposition_table.get(&number).unwrap();
            continue;
        }
        let current_number = number;
        let mut current_amount = 0;
        while right[right_index] <= current_number {
            if right[right_index] == current_number {
                current_amount += 1;
            }
            right_index += 1;
        }
        transposition_table.insert(current_number, current_amount);
        result += current_number * current_amount;
    }
    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
