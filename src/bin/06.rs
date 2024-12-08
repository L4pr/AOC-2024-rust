use advent_of_code::make_grid;
use std::collections::HashSet;
use std::thread;

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
    (
        (current_location.0 as i32 + DIRECTIONS[direction].0) as usize,
        (current_location.1 as i32 + DIRECTIONS[direction].1) as usize,
    )
}

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

// Single Threaded part 2 solution
// pub fn part_two(input: &str) -> Option<u32> {
//     let mut grid = make_grid(input);
//     grid.push(vec!['?'; grid[0].len()]);
//     grid.insert(0, vec!['?'; grid[0].len()]);
//     for i in 0..grid.len() {
//         grid[i].push('?');
//         grid[i].insert(0, '?');
//     }
//     let mut start_location = (0, 0);
//     for i in 0..grid.len() {
//         for j in 0..grid.len() {
//             if grid[i][j] == '^' {
//                 start_location = (i, j);
//                 break;
//             }
//         }
//         if start_location.0 > 0 {
//             break;
//         }
//     }
//
//
//     let mut direction: usize = 0;
//     let mut current_location = start_location;
//     let mut visited: HashSet<(usize, usize)> = HashSet::new();
//     visited.insert(current_location);
//     while grid[current_location.0][current_location.1] != '?' {
//         let next = move_in_direction(direction, &current_location);
//         visited.insert(current_location);
//         if grid[next.0][next.1] == '#' {
//             direction = (direction + 1) % 4;
//         } else {
//             current_location = next;
//         }
//     }
//     visited.remove(&current_location);
//
//     let mut result = 0;
//     for location in visited {
//         grid[location.0][location.1] = '#';
//         let mut visited: HashSet<(usize, (usize, usize))> = HashSet::new();
//         let mut current_location = start_location;
//         let mut direction: usize = 0;
//         while grid[current_location.0][current_location.1] != '?' {
//             if visited.contains(&(direction, current_location)) {
//                 result += 1;
//                 break;
//             }
//             visited.insert((direction, current_location));
//             let next = move_in_direction(direction, &current_location);
//             if grid[next.0][next.1] == '#' {
//                 direction = (direction + 1) % 4;
//             } else {
//                 current_location = next;
//             }
//         }
//         grid[location.0][location.1] = '.';
//     }
//
//     Some(result)
// }

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

    let mut direction: usize = 0;
    let mut current_location = start_location;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(current_location);
    while grid[current_location.0][current_location.1] != '?' {
        let next = move_in_direction(direction, &current_location);
        visited.insert(current_location);
        if grid[next.0][next.1] == '#' {
            direction = (direction + 1) % 4;
        } else {
            current_location = next;
        }
    }
    visited.remove(&current_location);

    let visited_vec: Vec<_> = visited.into_iter().collect();
    let cores = 12;

    let mut chunks: Vec<Vec<(usize, usize)>> = vec![Vec::new(); cores];
    for i in 0..visited_vec.len() {
        chunks[i % cores].push(visited_vec[i]);
    }

    // let chunk_size = visited_vec.len() / cores;
    // let chunks: Vec<_> = visited_vec
    //     .chunks(chunk_size)
    //     .map(|chunk| chunk.to_vec())
    //     .collect();

    let mut handles = Vec::new();

    for chunk in chunks {
        let mut grid_clone = grid.clone();

        let handle = thread::spawn(move || {
            let mut local_result = 0;
            for location in chunk {
                grid_clone[location.0][location.1] = '#';

                let mut visited: HashSet<(usize, (usize, usize))> = HashSet::new();
                let mut current_location = start_location;
                let mut direction: usize = 0;

                while grid_clone[current_location.0][current_location.1] != '?' {
                    if visited.contains(&(direction, current_location)) {
                        local_result += 1;
                        break;
                    }
                    visited.insert((direction, current_location));
                    let next = move_in_direction(direction, &current_location);
                    if grid_clone[next.0][next.1] == '#' {
                        direction = (direction + 1) % 4;
                    } else {
                        current_location = next;
                    }
                }

                grid_clone[location.0][location.1] = '.';
            }
            local_result
        });

        handles.push(handle);
    }

    let result: u32 = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .sum();

    Some(result)
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
