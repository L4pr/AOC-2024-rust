use std::cmp::Ordering;
use std::collections::HashMap;
use advent_of_code::get_lines;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;

    let lines = get_lines(input);
    let mut second_part = false;
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    for line in lines {
        if !second_part {
            if line == "" {
                second_part = true;
                continue;
            }
            let parts: Vec<u32> = line.split("|").into_iter().map(|s| s.parse::<u32>().unwrap()).collect();
            if rules.contains_key(&parts[0]) {
                rules.get_mut(&parts[0]).unwrap().push(parts[1]);
            } else {
                let temp_vec = vec![parts[1]];
                rules.insert(parts[0], temp_vec);
            }
            continue;
        }

        let mut numbers: Vec<u32> = line.split(",").into_iter().map(|s| s.parse::<u32>().unwrap()).collect();
        let mut numbers_been: Vec<u32> = Vec::new();
        let mut is_correct = true;
        while !numbers.is_empty() {
            let temp = numbers.remove(0);
            numbers_been.push(temp);
            if !rules.contains_key(&temp) {
                continue;
            }
            if rules.get(&temp).unwrap().iter().any(|x| numbers_been.contains(x)) {
                is_correct = false;
                break;
            }
        }
        if is_correct {
            result += numbers_been[numbers_been.len() / 2];
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;

    let lines = get_lines(input);
    let mut second_part = false;
    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    for line in lines {
        if !second_part {
            if line == "" {
                second_part = true;
                continue;
            }
            let parts: Vec<u32> = line.split("|").into_iter().map(|s| s.parse::<u32>().unwrap()).collect();
            if rules.contains_key(&parts[0]) {
                rules.get_mut(&parts[0]).unwrap().push(parts[1]);
            } else {
                let temp_vec = vec![parts[1]];
                rules.insert(parts[0], temp_vec);
            }
            continue;
        }

        let mut numbers: Vec<u32> = line.split(",").into_iter().map(|s| s.parse::<u32>().unwrap()).collect();
        let mut numbers_been: Vec<u32> = Vec::new();
        let mut is_correct = true;
        while !numbers.is_empty() {
            let temp = numbers.remove(0);
            numbers_been.push(temp);
            if !rules.contains_key(&temp) {
                continue;
            }
            if rules.get(&temp).unwrap().iter().any(|x| numbers_been.contains(x)) {
                is_correct = false;
                break;
            }
        }
        if is_correct {
            continue;
        }
        let mut numbers: Vec<u32> = line.split(",").into_iter().map(|s| s.parse::<u32>().unwrap()).collect();

        numbers.sort_by(|a, b| {
            if a == b {
                Ordering::Equal
            } else if rules.contains_key(a) && rules.get(a).unwrap().contains(b) {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        });

        result += numbers[numbers.len() / 2];
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
