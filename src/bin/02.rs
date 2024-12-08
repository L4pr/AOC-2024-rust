use advent_of_code::get_lines;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = get_lines(input);
    let mut array = Vec::new();

    lines.into_iter().for_each(|line| {
        array.push(
            line.split_whitespace()
                .collect::<Vec<&str>>()
                .into_iter()
                .map(|number| number.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        )
    });

    Some(array.into_iter().map(|temp| is_correct(&temp)).sum::<u32>())

    // for line in lines {
    //     let parts: Vec<&str> = line.split_whitespace().collect();
    //     array.push(parts.into_iter().map(|number| number.parse::<i32>().unwrap()).collect::<Vec<i32>>());
    // }

    // let mut result = 0;
    // for i in 0..array.len() {
    //     if array[i][0] < array[i][1] {
    //         if all_increasing(&array[i]) {
    //             result += 1;
    //         }
    //     } else {
    //         if all_decreasing(&array[i]) {
    //             result += 1;
    //         }
    //     }
    // }
    // Some(result)
}

fn is_correct(input: &Vec<i32>) -> u32 {
    if input[0] < input[1] {
        if all_increasing(input) {
            return 1;
        }
    } else if all_decreasing(input) {
        return 1;
    }
    0
}

fn all_decreasing(input: &Vec<i32>) -> bool {
    for i in 0..input.len() - 1 {
        let temp = input[i] - input[i + 1];
        if !(1..=3).contains(&temp) {
            return false;
        }
    }
    true
}

fn all_increasing(input: &Vec<i32>) -> bool {
    for i in 0..input.len() - 1 {
        let temp = input[i + 1] - input[i];
        if !(1..=3).contains(&temp) {
            return false;
        }
    }
    true
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = get_lines(input);
    let mut array = Vec::new();
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        array.push(
            parts
                .into_iter()
                .map(|number| number.parse::<i32>().unwrap())
                .collect::<Vec<i32>>(),
        );
    }
    let mut result = 0;
    for i in 0..array.len() {
        if array[i][0] < array[i][1] {
            if all_increasing(&array[i]) {
                result += 1;
                continue;
            }
        } else if all_decreasing(&array[i]) {
            result += 1;
            continue;
        }
        for j in 0..array[i].len() {
            let temp = array[i][j];
            array[i].remove(j);
            if array[i][0] < array[i][1] {
                if all_increasing(&array[i]) {
                    result += 1;
                    break;
                }
            } else if all_decreasing(&array[i]) {
                result += 1;
                break;
            }
            array[i].insert(j, temp);
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
