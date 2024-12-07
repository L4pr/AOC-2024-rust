use advent_of_code::get_lines;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = get_lines(input);
    let mut result = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(": ").collect();
        let goal = parts[0].parse::<i64>().unwrap();
        let array: Vec<i64> = parts[1]
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        if try_calculations_rev(goal, &array, array.len() - 1) {
            result += goal;
        }
    }
    Some(result as u64)
}

fn try_calculations_rev(current: i64, array: &Vec<i64>, index: usize) -> bool {
    let value = array[index];
    if index == 0 {
        return current == value;
    }
    if current % value == 0 {
        if try_calculations_rev(current / value, array, index - 1) {
            return true;
        }
    }

    if value > current {
        return false;
    }

    try_calculations_rev(current - value, array, index - 1)
}

fn try_calculations_rev_part2(current: i64, array: &Vec<i64>, index: usize) -> bool {
    let value = array[index];
    if index == 0 {
        return current == value;
    }
    if current % value == 0 {
        if try_calculations_rev_part2(current / value, array, index - 1) {
            return true;
        }
    }

    let power = 10i64.pow(value.ilog10() + 1);
    if current % power == value {
        if try_calculations_rev_part2(current / power, array, index - 1) {
            return true;
        }
    }

    if value > current {
        return false;
    }

    try_calculations_rev_part2(current - value, array, index - 1)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = get_lines(input);
    let mut result = 0;
    for line in lines {
        let parts: Vec<&str> = line.split(": ").collect();
        let goal = parts[0].parse::<i64>().unwrap();
        let array: Vec<i64> = parts[1]
            .split_whitespace()
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.parse::<i64>().unwrap())
            .collect();
        if try_calculations_rev_part2(goal, &array, array.len() - 1) {
            result += goal;
        }
    }
    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
