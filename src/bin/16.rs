use advent_of_code::{make_grid, move_in_direction, PriorityQueue};
use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = make_grid(input);

    let end = (1, grid.len() as i32 - 2);
    let start = (grid.len() as i32 - 2, 1);

    let comparator = |a: &((i32, i32), usize, u32), b: &((i32, i32), usize, u32)| a.2.cmp(&b.2);
    let mut queue = PriorityQueue::new(comparator);

    let mut location_been: HashMap<((i32, i32), usize), u32> = HashMap::new();
    queue.push((start, 1, 0));

    while !queue.is_empty() {
        let temp = queue.pop().unwrap();

        if grid[temp.0 .0 as usize][temp.0 .1 as usize] == '#' {
            continue;
        }
        if let Some(value) = location_been.get(&(temp.0, temp.1)) {
            if value > &temp.2 {
                location_been.insert((temp.0, temp.1), temp.2);
            }
            continue;
        } else {
            location_been.insert((temp.0, temp.1), temp.2);
        }

        if temp.0 == end {
            return Some(temp.2);
        }

        let next_location = move_in_direction(temp.1, &temp.0);

        for i in 0..4 {
            if i == temp.1 {
                queue.push((next_location, temp.1, temp.2 + 1));
            } else if (i + 2) % 4 == temp.1 {
                continue;
            } else {
                queue.push((temp.0, i, temp.2 + 1000));
            }
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = make_grid(input);

    let end = (1, grid.len() as i32 - 2);
    let start = (grid.len() as i32 - 2, 1);

    let comparator = |a: &((i32, i32), usize, u32), b: &((i32, i32), usize, u32)| a.2.cmp(&b.2);
    let mut queue = PriorityQueue::new(comparator);

    let mut location_been: HashMap<((i32, i32), usize), u32> = HashMap::new();
    queue.push((start, 1, 0));

    let mut result = 1u32 << 31;

    let mut found_end = Vec::new();

    while !queue.is_empty() {
        let temp = queue.pop().unwrap();

        if grid[temp.0 .0 as usize][temp.0 .1 as usize] == '#' {
            continue;
        }
        if let Some(value) = location_been.get(&(temp.0, temp.1)) {
            if value > &temp.2 {
                location_been.insert((temp.0, temp.1), temp.2);
            }
            continue;
        } else {
            location_been.insert((temp.0, temp.1), temp.2);
        }

        if temp.0 == end {
            if temp.2 < result {
                result = temp.2;
                found_end.push((temp.0, temp.1, temp.2));
            }
            continue;
        }

        let next_location = move_in_direction(temp.1, &temp.0);

        for i in 0..4 {
            if i == temp.1 {
                queue.push((next_location, temp.1, temp.2 + 1));
            } else if (i + 2) % 4 == temp.1 {
                continue;
            } else {
                queue.push((temp.0, i, temp.2 + 1000));
            }
        }
    }

    let found_end: Vec<_> = found_end.into_iter().filter(|o| o.2 == result).collect();

    let min_cost = result;
    let mut shortest_path_positions = std::collections::HashSet::new();
    let mut to_explore = Vec::new();
    for i in 0..found_end.len() {
        to_explore.push((end, found_end[i].1, min_cost));
    }

    while let Some((pos, dir, cost)) = to_explore.pop() {
        shortest_path_positions.insert(pos);

        let prev_pos = reverse_move_in_direction(dir, &pos);
        if let Some(prev_cost) = location_been.get(&(prev_pos, dir)) {
            if *prev_cost + 1 == cost {
                to_explore.push((prev_pos, dir, *prev_cost));
            }
        }

        for prev_dir_candidate in 0..4 {
            if prev_dir_candidate != dir && (prev_dir_candidate + 2) % 4 != dir {
                // This means we turned from prev_dir_candidate to dir with cost +1000
                if let Some(prev_cost) = location_been.get(&(pos, prev_dir_candidate)) {
                    if *prev_cost + 1000 == cost {
                        to_explore.push((pos, prev_dir_candidate, *prev_cost));
                    }
                }
            }
        }
    }

    Some(shortest_path_positions.len() as u32)
}

fn reverse_move_in_direction(dir: usize, pos: &(i32, i32)) -> (i32, i32) {
    match dir {
        0 => (pos.0 + 1, pos.1), // Opposite of Up is Down
        1 => (pos.0, pos.1 - 1), // Opposite of Right is Left
        2 => (pos.0 - 1, pos.1), // Opposite of Down is Up
        3 => (pos.0, pos.1 + 1), // Opposite of Left is Right
        _ => *pos,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
