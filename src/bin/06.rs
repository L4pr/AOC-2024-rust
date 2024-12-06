use std::collections::HashSet;
use advent_of_code::make_grid;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = make_grid(input);
    grid.push(vec!['?'; grid[0].len()]);
    grid.insert(0, vec!['?'; grid[0].len()]);
    for i in 0..grid.len() {
        grid[i].push('?');
        grid[i].insert(0, '?');
    }
    let mut visited = Vec::new();
    for _i in 0..grid.len() {
        visited.push(vec![0; grid[0].len()]);
    }
    let mut current_location = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            if grid[i][j] == '^' {
                current_location = (i, j);
                visited[i][j] = 1;
                break;
            }
        }
        if current_location.0 > 0 {
            break;
        }
    }
    let mut direction: usize = 0;
    while grid[current_location.0][current_location.1] != '?' {
        let next = move_in_direction(direction, &current_location);
        visited[current_location.0][current_location.1] = 1;
        if grid[next.0][next.1] == '#' {
            direction = (direction + 1) % 4;
        } else {
            current_location = next;
        }
    }

    let mut result = 0;

    for i in 0..visited.len() {
        result += visited[i].iter().filter(|f| **f == 1).count();
    }

    Some(result as u32)
}

fn move_in_direction(direction: usize, current_location: &(usize, usize)) -> (usize, usize) {
    ((current_location.0 as i32 + DIRECTIONS[direction].0) as usize, (current_location.1 as i32 + DIRECTIONS[direction].1) as usize)
}

const DIRECTIONS: [(i32, i32); 4] = [
    (-1, 0),
    (0, 1),
    (1, 0),
    (0, -1),
];

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = make_grid(input);
    grid.push(vec!['?'; grid[0].len()]);
    grid.insert(0, vec!['?'; grid[0].len()]);
    for i in 0..grid.len() {
        grid[i].push('?');
        grid[i].insert(0, '?');
    }
    let mut start_location = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            if grid[i][j] == '^' {
                start_location = (i, j);
                break;
            }
        }
        if start_location.0 > 0 {
            break;
        }
    }
    let mut result = 0;
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            if grid[i][j] != '.' {
                continue;
            }
            grid[i][j] = '#';
            let mut visited: HashSet<(usize, (usize, usize))> = HashSet::new();
            let mut current_location = start_location;
            let mut direction: usize = 0;
            while grid[current_location.0][current_location.1] != '?' {
                if visited.contains(&(direction, current_location)) {
                    result += 1;
                    break;
                }
                visited.insert((direction, current_location));
                let next = move_in_direction(direction, &current_location);
                if grid[next.0][next.1] == '#' {
                    direction = (direction + 1) % 4;
                } else {
                    current_location = next;
                }
            }
            grid[i][j] = '.';
        }
    }

    Some(result as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
