use std::collections::HashSet;
use advent_of_code::get_lines;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = get_lines(input);

    let max_x;
    let max_y;
    if lines.len() > 16 {
        max_x = 101;
        max_y = 103;
    } else {
        max_x = 11;
        max_y = 7;
    }

    let mut quadrants = vec![0; 4];
    for line in lines {
        let split1: Vec<&str> = line.split_whitespace().collect();
        let coordinates: Vec<i32> = split1[0].split("=").into_iter().last().unwrap().split(",").into_iter().map(|s| s.parse::<i32>().unwrap()).collect();
        let velocity: Vec<i32> = split1[1].split("=").into_iter().last().unwrap().split(",").into_iter().map(|s| s.parse::<i32>().unwrap()).collect();

        let mut new_x = (coordinates[0] + (velocity[0] * 100)) % max_x;
        let mut new_y = (coordinates[1] + (velocity[1] * 100)) % max_y;
        if new_x < 0 {
            new_x += max_x;
        }
        if new_y < 0 {
            new_y += max_y;
        }
        let middle_x = max_x / 2;
        let middle_y = max_y / 2;
        if new_x < middle_x {
            if new_y < middle_y {
                quadrants[0] += 1;
            }
            if new_y > middle_y {
                quadrants[1] += 1;
            }
        } else if new_x > middle_x {
            if new_y < middle_y {
                quadrants[2] += 1;
            }
            if new_y > middle_y {
                quadrants[3] += 1;
            }
        }
    }
    let result = quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3];
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = get_lines(input);

    let max_x;
    let max_y;
    if lines.len() > 16 {
        max_x = 101;
        max_y = 103;
    } else {
        max_x = 11;
        max_y = 7;
    }

    let mut all_coordinates = Vec::new();
    let mut all_velocities = Vec::new();

    for line in lines {
        let split1: Vec<&str> = line.split_whitespace().collect();
        let coordinates: Vec<i32> = split1[0].split("=").into_iter().last().unwrap().split(",").into_iter().map(|s| s.parse::<i32>().unwrap()).collect();
        let velocity: Vec<i32> = split1[1].split("=").into_iter().last().unwrap().split(",").into_iter().map(|s| s.parse::<i32>().unwrap()).collect();

        all_coordinates.push(coordinates);
        all_velocities.push(velocity);
    }

    let mut result = 0;

    for i in 0..10000 {
        do_blink(&mut all_coordinates, &mut all_velocities, max_x, max_y);
        if i > 4000 {
            let mut all_coords = HashSet::new();
            let mut all_different = true;
            for j in 0..all_coordinates.len() {
                if !all_coords.insert(all_coordinates[j].clone()) {
                    all_different = false;
                }
            }
            if all_different {
                result = i + 1;
                break;
            }
        }
    }


    // print the grid with the christmas tree
    // let mut grid: Vec<Vec<char>> = vec![vec!['.'; max_y as usize]; max_x as usize];
    // for j in 0..all_coordinates.len() {
    //     grid[all_coordinates[j][0] as usize][all_coordinates[j][1] as usize] = '#';
    // }
    //
    // grid.iter().for_each(|row| {
    //     row.iter().for_each(|&c| print!("{}", c));
    //     println!();
    // });

    Some(result)
}

fn do_blink(all_coordinates: &mut Vec<Vec<i32>>, all_velocities: &mut Vec<Vec<i32>>, max_x: i32, max_y: i32) {
    for j in 0..all_coordinates.len() {
        let new_x = (all_coordinates[j][0] + (all_velocities[j][0] + max_x)) % max_x;
        all_coordinates[j][0] = new_x;
        let new_y = (all_coordinates[j][1] + (all_velocities[j][1] + max_y)) % max_y;
        all_coordinates[j][1] = new_y;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4005));
    }
}
