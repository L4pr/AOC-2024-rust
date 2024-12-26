use std::collections::HashMap;
use advent_of_code::{get_lines, move_in_direction, CollectVec, PriorityQueue};

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = get_lines(input);
    let size;
    if lines.len() > 100 {
        size = 71;
    } else {
        size = 7;
    }

    let mut grid = vec![vec![true; size]; size];
    let mut bytes = 0;
    for i in 0..lines.len() {
        bytes += 1;

        let parts = lines[i].split(",").into_iter().map(|x| x.parse::<usize>().unwrap()).collect_vec();
        grid[parts[0]][parts[1]] = false;

        if size == 7 && bytes >= 12 {
            break;
        }

        if bytes >= 1024 {
            break;
        }
    }

    grid.push(vec![false; size]);
    grid.insert(0, vec![false; size]);

    for i in 0..grid.len() {
        grid[i].push(false);
        grid[i].insert(0, false);
    }

    let end = (grid.len() as i32 - 2, grid.len() as i32 - 2);
    let start = (1, 1);

    let comparator = |a: &((i32, i32), u32), b: &((i32, i32),  u32)| a.1.cmp(&b.1);
    let mut queue = PriorityQueue::new(comparator);

    let mut location_been: HashMap<(i32, i32), u32> = HashMap::new();
    queue.push((start, 0));

    while !queue.is_empty() {
        let temp = queue.pop().unwrap();

        if !grid[temp.0 .0 as usize][temp.0 .1 as usize] {
            continue;
        }

        if let Some(value) = location_been.get(&temp.0) {
            if value > &temp.1 {
                location_been.insert(temp.0, temp.1);
            } else {
                continue;
            }
        } else {
            location_been.insert(temp.0, temp.1);
        }

        if temp.0 == end {
            return Some(temp.1);
        }

        for i in 0..4 {
            let next_location = move_in_direction(i, &temp.0);
            queue.push((next_location, temp.1 + 1));
        }
    }

    None
}

fn print_grid(grid: &Vec<Vec<bool>>) {
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
}

pub fn part_two(input: &str) -> Option<String> {
    let lines = get_lines(input);
    let size;
    if lines.len() > 100 {
        size = 71;
    } else {
        size = 7;
    }

    let mut grid = vec![vec![true; size]; size];
    let mut bytes = 0;

    grid.push(vec![false; size]);
    grid.insert(0, vec![false; size]);

    for i in 0..grid.len() {
        grid[i].push(false);
        grid[i].insert(0, false);
    }

    for i in 0..lines.len() {
        bytes += 1;

        let parts = lines[i].split(",").into_iter().map(|x| x.parse::<usize>().unwrap()).collect_vec();
        grid[parts[0] + 1][parts[1] + 1] = false;

        if let Some(result) = find_route(&grid) {
            continue;
        } else {
            return Some(String::from(lines[i]));
        }
    }

    None
}

fn find_route(grid: &Vec<Vec<bool>>) -> Option<u32> {
    let end = (grid.len() as i32 - 2, grid.len() as i32 - 2);
    let start = (1, 1);

    let comparator = |a: &((i32, i32), u32), b: &((i32, i32),  u32)| a.1.cmp(&b.1);
    let mut queue = PriorityQueue::new(comparator);

    let mut location_been: HashMap<(i32, i32), u32> = HashMap::new();
    queue.push((start, 0));

    while !queue.is_empty() {
        let temp = queue.pop().unwrap();

        if !grid[temp.0 .0 as usize][temp.0 .1 as usize] {
            continue;
        }

        if let Some(value) = location_been.get(&temp.0) {
            if value > &temp.1 {
                location_been.insert(temp.0, temp.1);
            } else {
                continue;
            }
        } else {
            location_been.insert(temp.0, temp.1);
        }

        if temp.0 == end {
            return Some(temp.1);
        }

        for i in 0..4 {
            let next_location = move_in_direction(i, &temp.0);
            queue.push((next_location, temp.1 + 1));
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
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("6,1")));
    }
}
