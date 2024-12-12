use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;
    let numbers: Vec<u32> = input
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let mut transposition_table: HashMap<(u32, u64), u64> = HashMap::new();
    for number in numbers {
        result += get_stone_amount_part2(25, number as u64, &mut transposition_table);
    }
    Some(result as u32)
}

fn get_stone_amount_part2(
    blinks_left: u32,
    number: u64,
    transposition_table: &mut HashMap<(u32, u64), u64>,
) -> u64 {
    if blinks_left == 0 {
        return 1;
    }
    if transposition_table.contains_key(&(blinks_left, number)) {
        return *transposition_table.get(&(blinks_left, number)).unwrap();
    }
    if number == 0 {
        let stone_amount = get_stone_amount_part2(blinks_left - 1, 1, transposition_table);
        transposition_table.insert((blinks_left, number), stone_amount);
        return stone_amount;
    }
    let digits = (number as f64).log10().floor() as u32 + 1;
    if digits % 2 == 0 {
        let half_digits = digits / 2;
        let divisor = 10u64.pow(half_digits);
        let left = number / divisor;
        let right = number % divisor;
        let stone_amount = get_stone_amount_part2(blinks_left - 1, left, transposition_table)
            + get_stone_amount_part2(blinks_left - 1, right, transposition_table);
        transposition_table.insert((blinks_left, number), stone_amount);
        return stone_amount;
    }
    let stone_amount = get_stone_amount_part2(blinks_left - 1, number * 2024, transposition_table);
    transposition_table.insert((blinks_left, number), stone_amount);
    stone_amount
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result: u64 = 0;
    let numbers: Vec<u32> = input
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let mut transposition_table: HashMap<(u32, u64), u64> = HashMap::new();
    for number in numbers {
        result += get_stone_amount_part2(75, number as u64, &mut transposition_table);
    }
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
