advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let parts = input.split("mul(").collect::<Vec<&str>>();
    let mut result: u32 = 0;
    for i in 0..parts.len() {
        let mut part = parts[i];
        if !part.contains(')') || !part.contains(',') {
            continue;
        }
        part = part.split(")").collect::<Vec<&str>>()[0];
        let left_and_right: Vec<&str> = part.split(",").collect();
        if left_and_right.len() != 2 {
            continue;
        }

        let left = match left_and_right[0].parse::<u32>() {
            Ok(number) => number,
            Err(_e) => continue,
        };
        let right = match left_and_right[1].parse::<u32>() {
            Ok(number) => number,
            Err(_e) => continue,
        };
        result += left * right;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let parts = input.split("mul(").collect::<Vec<&str>>();
    let mut result: u32 = 0;
    let mut add = true;
    for i in 0..parts.len() {
        let current_add = add;
        let mut part = parts[i];
        let contains_do = part.contains("do()");
        let contains_dont = part.contains("don't()");
        if contains_do && contains_dont {
            add = part
                .split("do()")
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .len()
                < part
                    .split("don't()")
                    .collect::<Vec<&str>>()
                    .last()
                    .unwrap()
                    .len();
        } else if contains_dont {
            add = false;
        } else if contains_do {
            add = true;
        }
        if !current_add {
            continue;
        }
        if !part.contains(')') || !part.contains(',') {
            continue;
        }
        part = part.split(")").collect::<Vec<&str>>()[0];
        let left_and_right: Vec<&str> = part.split(",").collect();
        if left_and_right.len() != 2 {
            continue;
        }

        let left = match left_and_right[0].parse::<u32>() {
            Ok(number) => number,
            Err(_e) => continue,
        };
        let right = match left_and_right[1].parse::<u32>() {
            Ok(number) => number,
            Err(_e) => continue,
        };
        result += left * right;
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
