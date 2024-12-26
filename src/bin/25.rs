use advent_of_code::{get_lines, CollectVec};

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = get_lines(input);

    let mut locks: Vec<Vec<u8>> = Vec::new();
    let mut keys: Vec<Vec<u8>> = Vec::new();

    let mut index = 0;
    loop {
        if index >= lines.len() {
            break;
        }
        if lines[index] == "....." {
            keys.push(get_locks_and_keys(&lines, index + 1));
        } else {
            locks.push(get_locks_and_keys(&lines, index + 1));
        }
        index += 8;
    }

    let mut result = 0;

    for i in 0..locks.len() {
        let lock = &locks[i];
        for j in 0..keys.len() {
            let mut correct = true;
            let key = &keys[j];
            for k in 0..5 {
                if lock[k] & key[k] != 0 {
                    correct = false;
                    break;
                }
            }
            if correct {
                result += 1;
            }
        }
    }
    Some(result)
}

fn get_locks_and_keys(lines: &Vec<&str>, start_index: usize) -> Vec<u8> {
    let mut result: Vec<u8> = vec![0; 5];

    let mut index = start_index;
    loop {
        if index >= start_index + 5 {
            break;
        }
        let line = lines[index];
        if line == "....." {
            index += 1;
            continue;
        }
        let chars = line.chars().collect_vec();
        for i in 0..chars.len() {
            if chars[i] == '.' {
                continue;
            } else {
                result[i] |= 1 << (4 - (index - start_index))
            }
        }
        index += 1;
    }
    result
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
