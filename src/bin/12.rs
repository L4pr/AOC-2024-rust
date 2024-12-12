use advent_of_code::{make_grid, move_in_direction};
use std::collections::HashSet;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = make_grid(input);
    grid.insert(0, vec!['.'; grid[0].len()]);
    grid.push(vec!['.'; grid[0].len()]);

    for i in 0..grid.len() {
        grid[i].push('.');
        grid[i].insert(0, '.');
    }
    let mut total_locations_been: HashSet<(i32, i32)> = HashSet::new();
    let mut result = 0;
    let grid_length: usize = grid.len();
    for i in 1..grid_length - 1 {
        for j in 1..grid_length - 1 {
            let location = (i as i32, j as i32);
            if total_locations_been.contains(&location) {
                continue;
            }
            let character = grid[i][j];
            let mut location_been: HashSet<(i32, i32)> = HashSet::new();
            let mut circumference = 0;
            let square_amount = get_locations_are(
                location,
                &grid,
                character,
                &mut location_been,
                &mut circumference,
            );
            result += square_amount * circumference;
            total_locations_been.extend(location_been);
        }
    }

    Some(result)
}

fn get_locations_are(
    location: (i32, i32),
    grid: &Vec<Vec<char>>,
    character: char,
    locations_been: &mut HashSet<(i32, i32)>,
    circumference: &mut u32,
) -> u32 {
    if grid[location.0 as usize][location.1 as usize] != character {
        *circumference += 1;
        return 0;
    }
    if locations_been.contains(&location) {
        return 0;
    }
    locations_been.insert(location);
    let mut result = 1;
    for i in 0..4 {
        result += get_locations_are(
            move_in_direction(i, &location),
            grid,
            character,
            locations_been,
            circumference,
        );
    }
    result
}

fn get_locations_are_part2(
    location: (i32, i32),
    grid: &Vec<Vec<char>>,
    character: char,
    locations_been: &mut HashSet<(i32, i32)>,
    locations_outside: &mut HashSet<((i32, i32), usize)>,
    direction: usize,
) -> u32 {
    if grid[location.0 as usize][location.1 as usize] != character {
        locations_outside.insert((location, direction));
        return 0;
    }
    if locations_been.contains(&location) {
        return 0;
    }
    locations_been.insert(location);
    let mut result = 1;
    for i in 0..4 {
        result += get_locations_are_part2(
            move_in_direction(i, &location),
            grid,
            character,
            locations_been,
            locations_outside,
            i,
        );
    }
    result
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = make_grid(input);
    grid.insert(0, vec!['.'; grid[0].len()]);
    grid.push(vec!['.'; grid[0].len()]);

    for i in 0..grid.len() {
        grid[i].push('.');
        grid[i].insert(0, '.');
    }
    let mut total_locations_been: HashSet<(i32, i32)> = HashSet::new();
    let mut result = 0;
    let grid_length: usize = grid.len();
    for i in 1..grid_length - 1 {
        for j in 1..grid_length - 1 {
            let location = (i as i32, j as i32);
            if total_locations_been.contains(&location) {
                continue;
            }
            let character = grid[i][j];
            let mut location_been: HashSet<(i32, i32)> = HashSet::new();
            let mut locations_outside = HashSet::new();
            let square_amount = get_locations_are_part2(
                location,
                &grid,
                character,
                &mut location_been,
                &mut locations_outside,
                i,
            );
            let mut lines = 0;
            while !locations_outside.is_empty() {
                lines += 1;
                let location = *locations_outside.iter().next().unwrap();
                remove_line(&mut locations_outside, location.0, location.1);
            }

            result += lines * square_amount;

            total_locations_been.extend(location_been);
        }
    }

    Some(result)
}

fn remove_line(
    locations_outside: &mut HashSet<((i32, i32), usize)>,
    location: (i32, i32),
    direction: usize,
) {
    if locations_outside.contains(&(location, direction)) {
        locations_outside.remove(&(location, direction));
    } else {
        return;
    }
    for i in 0..4 {
        if i == direction {
            continue;
        }
        remove_line(
            locations_outside,
            move_in_direction(i, &location),
            direction,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
