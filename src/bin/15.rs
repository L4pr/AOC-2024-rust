use std::collections::HashSet;
use advent_of_code::{get_lines, CollectVec};

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = get_lines(input);
    let mut grid = Vec::new();
    let mut movements = Vec::new();

    let mut is_grid = true;
    for line in lines {
        if line.is_empty() {
            is_grid = false;
        }
        if is_grid {
            grid.push(line.chars().collect_vec());
        } else {
            movements.extend(line.chars().collect_vec());
        }
    }

    let mut coordinates = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '@' {
                grid[i][j] = '.';
                coordinates = (i as i32, j as i32);
                break;
            }
        }
        if coordinates != (0, 0) {
            break;
        }
    }

    for movement in movements {
        coordinates = do_move(&mut grid, coordinates, movement);
    }

    let mut result = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'O' {
                result += (100 * i + j) as u32;
            }
        }
    }
    Some(result)
}

fn do_move(grid: &mut Vec<Vec<char>>, coordinates: (i32, i32), movement: char) -> (i32, i32) {
    let (dx, dy) = match movement {
        '<' => (0, -1),
        '>' => (0, 1),
        '^' => (-1, 0),
        _ => (1, 0),
    };
    let (new_x, new_y) = (coordinates.0 + dx, coordinates.1 + dy);
    let next_char = grid[new_x as usize][new_y as usize];
    if next_char == '.' {
        return (new_x, new_y);
    }
    if next_char == '#' {
        return coordinates;
    }
    let (mut next_x, mut next_y) = (new_x, new_y);
    loop {
        next_x += dx;
        next_y += dy;
        let next_char = grid[next_x as usize][next_y as usize];
        if next_char == '#' {
            return coordinates;
        }
        if next_char == '.' {
            grid[next_x as usize][next_y as usize] = 'O';
            grid[new_x as usize][new_y as usize] = '.';
            return (new_x, new_y);
        }
    }
}


pub fn part_two(input: &str) -> Option<u32> {
    let lines = get_lines(input);
    let mut grid = Vec::new();
    let mut movements = Vec::new();

    let mut is_grid = true;
    for line in lines {
        if line.is_empty() {
            is_grid = false;
        }
        if is_grid {
            let mut characters = Vec::new();
            for character in line.chars() {
                let new_chars = match character {
                    '#' => vec!['#', '#'],
                    'O' => vec!['[', ']'],
                    '@' => vec!['@', '.'],
                    _ => vec!['.', '.']
                };
                characters.extend(new_chars);
            }
            grid.push(characters);
        } else {
            movements.extend(line.chars().collect_vec());
        }
    }

    let mut coordinates = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '@' {
                grid[i][j] = '.';
                coordinates = (i as i32, j as i32);
                break;
            }
        }
        if coordinates != (0, 0) {
            break;
        }
    }

    for movement in movements {
        coordinates = do_move_part2(&mut grid, coordinates, movement);
    }

    let mut result = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '[' {
                result += (100 * i + j) as u32;
            }
        }
    }
    Some(result)
}

fn do_move_part2(grid: &mut Vec<Vec<char>>, coordinates: (i32, i32), movement: char) -> (i32, i32) {
    let (dx, dy) = match movement {
        '<' => (0, -1),
        '>' => (0, 1),
        '^' => (-1, 0),
        _ => (1, 0),
    };
    let (new_x, new_y) = (coordinates.0 + dx, coordinates.1 + dy);
    let next_char = grid[new_x as usize][new_y as usize];
    if next_char == '.' {
        return (new_x, new_y);
    }
    if next_char == '#' {
        return coordinates;
    }
    let mut boxes_influenced: HashSet<((i32, i32), char)> = HashSet::new();
    if movement == '^' || movement == 'v' {
        if !get_boxes_up_down(grid, coordinates, dx, dy, &mut boxes_influenced) {
            return coordinates;
        }
    } else {
        if !get_boxes_left_right(grid, coordinates, dx, dy, &mut boxes_influenced) {
            return coordinates;
        }
    }

    for ib in &boxes_influenced {
        grid[ib.0.0 as usize][ib.0.1 as usize] = '.';
    }

    for ib in &boxes_influenced {
        grid[(ib.0.0 + dx) as usize][(ib.0.1 + dy) as usize] = ib.1;
    }

    (new_x, new_y)
}

fn get_boxes_left_right(grid: &Vec<Vec<char>>, coordinates: (i32, i32), dx: i32, dy: i32, boxes_influenced: &mut HashSet<((i32, i32), char)>) -> bool {
    boxes_influenced.insert((coordinates, grid[coordinates.0 as usize][coordinates.1 as usize]));

    let (new_x, new_y) = (coordinates.0 + dx, coordinates.1 + dy);
    let next_char = grid[new_x as usize][new_y as usize];
    if next_char == '.' {
        return true;
    }
    if next_char == '#' {
        return false;
    }
    get_boxes_left_right(grid, (new_x, new_y), dx, dy, boxes_influenced)
}

fn get_boxes_up_down(grid: &mut Vec<Vec<char>>, coordinates: (i32, i32), dx: i32, dy: i32, boxes_influenced: &mut HashSet<((i32, i32), char)>) -> bool {
    if !boxes_influenced.insert((coordinates, grid[coordinates.0 as usize][coordinates.1 as usize])) {
        return true;
    }

    let (new_x, new_y) = (coordinates.0 + dx, coordinates.1 + dy);
    let next_char = grid[new_x as usize][new_y as usize];
    if next_char == '#' {
        return false;
    }
    if next_char == '.' {
        return true;
    }
    return if next_char == ']' {
        get_boxes_up_down(grid, (new_x, new_y), dx, dy, boxes_influenced) && get_boxes_up_down(grid, (new_x, new_y - 1), dx, dy, boxes_influenced)
    } else {
        get_boxes_up_down(grid, (new_x, new_y), dx, dy, boxes_influenced) && get_boxes_up_down(grid, (new_x, new_y + 1), dx, dy, boxes_influenced)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
