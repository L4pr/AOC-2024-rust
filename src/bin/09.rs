use advent_of_code::get_lines;
use std::collections::VecDeque;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = get_lines(input);
    let array: VecDeque<u32> = lines[0]
        .chars()
        .into_iter()
        .map(|s| (s as u8 - '0' as u8) as u32)
        .collect();
    let mut result: u64 = 0;

    let mut number_array: VecDeque<i32> = VecDeque::new();
    let mut is_dots = false;
    let mut index = 0;
    for number in array {
        for _ in 0..number {
            if is_dots {
                number_array.push_back(-1);
            } else {
                number_array.push_back(index);
            }
        }
        if !is_dots {
            index += 1;
        }
        is_dots = !is_dots;
    }

    index = 0;
    while !number_array.is_empty() {
        let front = number_array.pop_front().unwrap();
        if front != -1 {
            result += (index * front) as u64;
            index += 1;
        } else {
            let mut back = -1;
            while back == -1 {
                if number_array.is_empty() {
                    return Some(result);
                }
                back = number_array.pop_back().unwrap();
            }
            result += (index * back) as u64;
            index += 1;
        }
    }

    // let mut index = 0;
    // let mut back_number = number_array.len() / 2;
    // let mut back_numbers_left = number_array.pop_back().unwrap();
    // let mut front_number = 0;
    // let mut front_left = number_array.pop_front().unwrap();
    // let mut is_dots = false;

    // while !number_array.is_empty() {
    //     if is_dots {
    //         if back_numbers_left == 0 {
    //             number_array.pop_back().unwrap();
    //             back_numbers_left = number_array.pop_back().unwrap();
    //             back_number -= 1;
    //         } else if front_left == 0 {
    //             is_dots = !is_dots;
    //             front_left = number_array.pop_front().unwrap();
    //             front_number += 1;
    //         } else {
    //             result += index * back_number;
    //             index += 1;
    //             back_numbers_left -= 1;
    //             front_left -= 1;
    //         }
    //     } else {
    //         if front_left == 0 {
    //             is_dots = !is_dots;
    //             front_left = number_array.pop_front().unwrap();
    //         } else {
    //             result += index * front_number;
    //             index += 1;
    //             front_left -= 1;
    //         }
    //     }
    // }
    //
    // loop {
    //     if is_dots {
    //         if back_numbers_left == 0 {
    //             while front_left > 0 {
    //                 result += index * front_number;
    //                 index += 1;
    //                 front_left -= 1;
    //             }
    //             break;
    //         } else if front_left == 0 {
    //             while back_numbers_left > 0 {
    //                 result += index * back_number;
    //                 index += 1;
    //                 back_numbers_left -= 1;
    //             }
    //             break;
    //         } else {
    //             result += index * back_number;
    //             index += 1;
    //             back_numbers_left -= 1;
    //             front_left -= 1;
    //         }
    //     } else {
    //         if front_left == 0 {
    //             is_dots = !is_dots;
    //         } else {
    //             result += index * front_number;
    //             index += 1;
    //             front_left -= 1;
    //         }
    //     }
    // }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = get_lines(input);

    let chars: Vec<char> = lines[0].chars().collect();
    let mut file_id = 0;
    let mut disk: Vec<Option<usize>> = Vec::new();
    let mut file_mode = true;

    for &c in &chars {
        let length = c.to_digit(10).unwrap() as usize;
        if file_mode {
            for _ in 0..length {
                disk.push(Some(file_id));
            }
            file_id += 1;
            file_mode = false;
        } else {
            for _ in 0..length {
                disk.push(None);
            }
            file_mode = true;
        }
    }

    let max_file_id = file_id - 1;

    let mut file_positions: Vec<Vec<usize>> = vec![Vec::new(); file_id];
    for (i, &block) in disk.iter().enumerate() {
        if let Some(fid) = block {
            file_positions[fid].push(i);
        }
    }

    for fid in (0..=max_file_id).rev() {
        let positions = &file_positions[fid];
        if positions.is_empty() {
            continue;
        }

        let file_start = *positions.first().unwrap();
        let file_len = positions.len();

        if let Some(free_start) = find_free_segment(&disk, file_start, file_len) {
            for &pos in positions {
                disk[pos] = None;
            }
            for (offset, &_pos) in positions.iter().enumerate() {
                disk[free_start + offset] = Some(fid);
            }

            let mut new_positions = Vec::with_capacity(file_len);
            for i in free_start..free_start + file_len {
                new_positions.push(i);
            }
            file_positions[fid] = new_positions;
        }
    }

    let mut result: u64 = 0;
    for (i, &block) in disk.iter().enumerate() {
        if let Some(fid) = block {
            result += (i as u64) * (fid as u64);
        }
    }

    Some(result)
}

fn find_free_segment(disk: &Vec<Option<usize>>, max_end: usize, length: usize) -> Option<usize> {
    // We want to find a run of `length` Nones that ends before `max_end`.
    // Simple approach: scan from 0 to max_end - length.

    if length == 0 {
        return None; // No need to move empty files (shouldn't happen).
    }

    let mut count = 0;
    let mut start_idx = 0;
    for i in 0..max_end {
        if disk[i].is_none() {
            count += 1;
            if count == 1 {
                start_idx = i;
            }
            if count == length {
                // Found a segment from start_idx to start_idx+length-1
                return Some(start_idx);
            }
        } else {
            count = 0;
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
