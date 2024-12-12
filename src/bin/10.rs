use advent_of_code::make_grid_int;
use std::collections::VecDeque;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = make_grid_int(input);

    let mut result = 0;

    grid.insert(0, vec![100; grid[0].len()]);
    grid.push(vec![100; grid[0].len()]);

    for i in 0..grid.len() {
        grid[i].push(100);
        grid[i].insert(0, 100);
    }

    for i in 0..grid.len() {
        for j in 0..grid.len() {
            if grid[i][j] == 0 {
                result += find_all_reachable_9s(&grid, (i as i32, j as i32));
            }
        }
    }

    Some(result)
}

fn find_all_reachable_9s(grid: &Vec<Vec<u32>>, location: (i32, i32)) -> u32 {
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    let mut locations_been: Vec<(i32, i32)> = Vec::new();
    let mut result = 0;

    queue.push_back(location);

    while !queue.is_empty() {
        let current_location = queue.pop_front().unwrap();
        if locations_been.contains(&current_location) {
            continue;
        }
        let current_grid_position = grid[current_location.0 as usize][current_location.1 as usize];
        if current_grid_position == 9 {
            result += 1;
        }
        locations_been.push(current_location);
        for i in 0..4 {
            let new_location = move_in_direction(i, &current_location);
            let temp = grid[new_location.0 as usize][new_location.1 as usize] as i32
                - current_grid_position as i32;
            if temp == 1 {
                queue.push_back(new_location);
            }
        }
    }

    result
}

fn move_in_direction(direction: usize, current_location: &(i32, i32)) -> (i32, i32) {
    (
        current_location.0 + DIRECTIONS[direction].0,
        current_location.1 + DIRECTIONS[direction].1,
    )
}

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn find_all_reachable_9s_part2(grid: &Vec<Vec<u32>>, location: (i32, i32)) -> u32 {
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    let mut result = 0;

    queue.push_back(location);

    while !queue.is_empty() {
        let current_location = queue.pop_front().unwrap();

        if grid[current_location.0 as usize][current_location.1 as usize] == 9 {
            result += 1;
        }
        for i in 0..4 {
            let new_location = move_in_direction(i, &current_location);
            let temp = grid[new_location.0 as usize][new_location.1 as usize] as i32
                - grid[current_location.0 as usize][current_location.1 as usize] as i32;
            if temp == 1 {
                queue.push_back(new_location);
            }
        }
    }

    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = make_grid_int(input);

    let mut result = 0;

    grid.insert(0, vec![100; grid[0].len()]);
    grid.push(vec![100; grid[0].len()]);

    for i in 0..grid.len() {
        grid[i].push(100);
        grid[i].insert(0, 100);
    }

    for i in 0..grid.len() {
        for j in 0..grid.len() {
            if grid[i][j] == 0 {
                result += find_all_reachable_9s_part2(&grid, (i as i32, j as i32));
            }
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
