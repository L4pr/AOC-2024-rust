use std::collections::HashMap;
use advent_of_code::{make_grid, move_in_direction, PriorityQueue};

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = make_grid(input);

    // find start
    let mut start = (0, 0);
    for i in 1..grid.len() - 1 {
        for j in 1..grid.len() - 1 {
            if grid[i][j] == 'S' {
                start = (i as i32, j as i32);
            }
        }
    }

    let mut route = Vec::new();
    find_route(&grid, start, &mut route);

    let mut result = 0;

    for i in 1..grid.len() - 1 {
        for j in 1..grid.len() - 1 {
            if grid[i][j] == '#' {
                let mut amount_char_around = 0;
                let mut locations = Vec::new();
                for k in 0..4 {
                    let next_location = move_in_direction(k, &(i as i32, j as i32));
                    if grid[next_location.0 as usize][next_location.1 as usize] == '#' {
                        amount_char_around += 1;
                    } else {
                        locations.push(next_location);
                    }
                }
                if amount_char_around != 2 {
                    continue;
                }

                let index1 = route.iter().position(|&x| x == locations[0]).unwrap();
                let index2 = route.iter().position(|&x| x == locations[1]).unwrap();
                // println!("{}, {}, {}", index1, index2, index1.abs_diff(index2));
                if index1.abs_diff(index2) - 2 >= 100 {
                    result += 1;
                }

            }
        }
    }
    Some(result)
}

fn find_route(grid: &Vec<Vec<char>>, start: (i32, i32), route: &mut Vec<(i32, i32)>) {
    let comparator = |a: &((i32, i32), u32), b: &((i32, i32),  u32)| a.1.cmp(&b.1);
    let mut queue = PriorityQueue::new(comparator);

    let mut location_been: HashMap<(i32, i32), u32> = HashMap::new();
    queue.push((start, 0));

    while !queue.is_empty() {
        let temp = queue.pop().unwrap();

        let character = grid[temp.0.0 as usize][temp.0.1 as usize];
        if character == '#' {
            continue;
        }

        if let Some(value) = location_been.get(&temp.0) {
            if value < &temp.1 {
                continue;
            } else {
                location_been.insert(temp.0, temp.1);
            }
        } else {
            location_been.insert(temp.0, temp.1);
        }

        route.push(temp.0);

        if character == 'E' {
            return;
        }

        for i in 0..4 {
            queue.push((move_in_direction(i, &temp.0), temp.1 + 1));
        }
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = make_grid(input);

    // find start
    let mut start = (0, 0);
    for i in 1..grid.len() - 1 {
        for j in 1..grid.len() - 1 {
            if grid[i][j] == 'S' {
                start = (i as i32, j as i32);
            }
        }
    }

    let mut route = Vec::new();
    find_route(&grid, start, &mut route);

    let jump_size: i32 = 20;

    let mut route_map: HashMap<(i32, i32), i32> = HashMap::new();
    for i in 0..route.len() {
        route_map.insert(route[i], i as i32);
    }

    let mut result = 0;
    for i in 0..route.len() {
        let p = route[i];
        for dx in -jump_size..=jump_size {
            for dy in -jump_size..=jump_size {
                if dx.abs() + dy.abs() > jump_size {
                    continue;
                }
                let new_point = (p.0 + dx, p.1 + dy);
                if let initial_cost = i as i32 {
                    if let Some(np_cost) = route_map.get(&new_point) {
                        let cheat_cost = dx.abs() + dy.abs();
                        if (initial_cost - np_cost - cheat_cost as i32) >= 100 {
                            result += 1;
                        }
                    }
                }
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
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }
}
