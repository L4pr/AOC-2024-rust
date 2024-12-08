use std::collections::HashMap;
use advent_of_code::make_grid;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = make_grid(input);
    let size = grid.len();

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for i in 0..size {
        for j in 0..size {
            let character = grid[i][j];
            if character != '.' {
                if antennas.contains_key(&character) {
                    antennas.get_mut(&character).unwrap().push((i as i32, j as i32))
                } else {
                    antennas.insert(character, vec![(i as i32, j as i32)]);
                }
            }
        }
    }

    let mut antinodes: Vec<Vec<u32>> = vec![vec![0; size]; size];

    for antenna in antennas {
        let array = antenna.1;
        for i in 0..array.len() - 1 {
            for j in i + 1..array.len() {
                let c1 = array[i];
                let c2 = array[j];
                let diff = (c2.0 - c1.0, c2.1 - c1.1);
                let a1 = (c1.0 - diff.0, c1.1 - diff.1);
                let a2 = (c2.0 + diff.0, c2.1 + diff.1);
                if is_inside(a1, size) {
                    antinodes[a1.0 as usize][a1.1 as usize] = 1;
                }
                if is_inside(a2, size) {
                    antinodes[a2.0 as usize][a2.1 as usize] = 1;
                }
            }
        }
    }

    let result: u32 = antinodes.iter().map(|f| f.iter().sum::<u32>()).sum();

    Some(result)
}

fn is_inside(coordinate: (i32, i32), size: usize) -> bool {
    if coordinate.0 < 0 || coordinate.1 < 0 {
        return false;
    }
    if coordinate.0 >= size as i32 || coordinate.1 >= size as i32 {
        return false;
    }
    true
}


pub fn part_two(input: &str) -> Option<u32> {
    let grid = make_grid(input);
    let size = grid.len();

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for i in 0..size {
        for j in 0..size {
            let character = grid[i][j];
            if character != '.' {
                if antennas.contains_key(&character) {
                    antennas.get_mut(&character).unwrap().push((i as i32, j as i32))
                } else {
                    antennas.insert(character, vec![(i as i32, j as i32)]);
                }
            }
        }
    }

    let mut antinodes: Vec<Vec<u32>> = vec![vec![0; size]; size];

    for antenna in antennas {
        let array = antenna.1;
        for i in 0..array.len() - 1 {
            for j in i + 1..array.len() {
                let c1 = array[i];
                let c2 = array[j];
                let diff = (c2.0 - c1.0, c2.1 - c1.1);
                let mut a1 = c1;

                while is_inside(a1, size) {
                    antinodes[a1.0 as usize][a1.1 as usize] = 1;
                    a1 = (a1.0 - diff.0, a1.1 - diff.1);
                }

                let mut a2 = c2;

                while is_inside(a2, size) {
                    antinodes[a2.0 as usize][a2.1 as usize] = 1;
                    a2 = (a2.0 + diff.0, a2.1 + diff.1);
                }
            }
        }
    }

    let result: u32 = antinodes.iter().map(|f| f.iter().sum::<u32>()).sum();

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
