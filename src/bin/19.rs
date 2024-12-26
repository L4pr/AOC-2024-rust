use std::collections::HashMap;
use advent_of_code::{get_lines, CollectVec};

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = get_lines(input);
    let words_parts = lines[0].split(", ").collect_vec();
    lines.remove(0);
    lines.remove(0);

    let mut result = 0;
    for line in lines {
        if can_word_be_made(line, &words_parts) {
            result += 1;
        }
    }


    Some(result)
}

fn can_word_be_made(word: &str, words_parts: &Vec<&str>) -> bool {
    if word == "" {
        return true;
    }
    for part in words_parts {
        if word.starts_with(part) {
            if can_word_be_made(word.split_once(part).unwrap().1, words_parts) {
                return true;
            }
        }
    }
    false
}

fn get_possible_ways_amount<'a>(word: &'a str, words_parts: &Vec<&str>, transpo_table: &mut HashMap<&'a str, u64>) -> u64 {
    if word == "" {
        return 1;
    }
    if let Some(value) = transpo_table.get(word) {
        return *value;
    }
    let mut result = 0;
    for part in words_parts {
        if word.starts_with(part) {
            result += get_possible_ways_amount(word.split_once(part).unwrap().1, words_parts, transpo_table);
        }
    }
    transpo_table.insert(word, result);
    result
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = get_lines(input);
    let words_parts = lines[0].split(", ").collect_vec();
    lines.remove(0);
    lines.remove(0);

    let mut result = 0;
    for line in lines {
        let mut transposition_table: HashMap<&str, u64> = HashMap::new();
        if can_word_be_made(line, &words_parts) {
            result += get_possible_ways_amount(line, &words_parts, &mut transposition_table);
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
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
