use std::collections::{HashMap, VecDeque};
use advent_of_code::{get_lines, move_in_direction, CollectVec};

advent_of_code::solution!(21);

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

const NUMERIC: [[u8; 5]; 6] = [
    [b' ', b' ', b' ', b' ', b' '],
    [b' ', b'7', b'8', b'9', b' '],
    [b' ', b'4', b'5', b'6', b' '],
    [b' ', b'1', b'2', b'3', b' '],
    [b' ', b' ', b'0', b'A', b' '],
    [b' ', b' ', b' ', b' ', b' '],
];

const DIRECTIONAL: [[u8; 5]; 4] = [
    [b' ', b' ', b' ', b' ', b' '],
    [b' ', b' ', b'^', b'A', b' '],
    [b' ', b'<', b'v', b'>', b' '],
    [b' ', b' ', b' ', b' ', b' ']];

pub fn part_one(input: &str) -> Option<u32> {
    let lines = get_lines(input);

    let mut cache = HashMap::new();
    let mut path_cache = HashMap::new();
    let mut result = 0;
    for line in lines {
        let sequence = line.as_bytes();
        let temp = find_shortest_sequence(sequence, 2, true, &mut cache, &mut path_cache);
        let number: i32 = line[..3].parse().unwrap();
        result += temp as i32 * number;
    }
    Some(result as u32)
}

fn find_shortest_sequence(
    sequence: &[u8],
    depth: usize,
    highest: bool,
    cache: &mut HashMap<(Vec<u8>, usize), usize>,
    path_cache: &mut HashMap<(u8, u8), Vec<Vec<u8>>>
) -> usize {
    let cache_key = (sequence.to_vec(), depth);
    if let Some(cached) = cache.get(&cache_key) {
        return *cached;
    }

    let mut cursor = b'A';
    let mut result = 0;
    for &c in sequence {
        let paths = find_shortest_paths(
            if highest { &NUMERIC } else { &DIRECTIONAL },
            cursor,
            c,
            path_cache,
        );
        if depth == 0 {
            // all paths have the same length
            result += paths[0].len();
        } else {
            result += paths
                .iter()
                .map(|path| find_shortest_sequence(path, depth - 1, false, cache, path_cache))
                .min().unwrap();

        }
        cursor = c;
    }

    cache.insert(cache_key, result);

    result
}

fn find_shortest_paths(keypad: &[[u8; 5]], start: u8, goal: u8, cache: &mut HashMap<(u8, u8), Vec<Vec<u8>>>) -> Vec<Vec<u8>> {
    if let Some(paths) = cache.get(&(start, goal)) {
        return paths.clone();
    }

    if start == goal {
        return vec![vec![b'A']];
    }

    let mut start_coords = (0, 0);
    let mut goal_coords = (0, 0);

    for i in 0..keypad.len() {
        for j in 0..keypad[i].len() {
            if keypad[i][j] == start {
                start_coords = (i as i32, j as i32);
            }
            if keypad[i][j] == goal {
                goal_coords = (i as i32, j as i32);
            }
        }
    }

    let mut dists = vec![[usize::MAX; 5]; keypad.len()];
    let mut queue = VecDeque::new();
    queue.push_back((start_coords.0, start_coords.1, 0));

    while let Some((x, y, steps)) = queue.pop_front() {
        dists[x as usize][y as usize] = steps;
        for i in 0..4 {
            let (nx, ny) = move_in_direction(i, &(x, y));
            if keypad[nx as usize][ny as usize] != b' '
                && dists[nx as usize][ny as usize] == usize::MAX
            {
                queue.push_back((nx, ny, steps + 1));
            }
        }
    }



    let mut paths = Vec::new();
    let mut stack = Vec::new();
    stack.push((goal_coords.0, goal_coords.1, vec![b'A']));
    while let Some((x, y, path)) = stack.pop() {
        if x == start_coords.0 && y == start_coords.1 {
            paths.push(path);
            continue;
        }
        for (i, (dx, dy)) in DIRS.iter().enumerate() {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if keypad[nx as usize][ny as usize] != b' ' && dists[nx as usize][ny as usize] < dists[x as usize][y as usize] {
                let c = match i {
                    0 => b'<',
                    1 => b'^',
                    2 => b'>',
                    3 => b'v',
                    _ => panic!(),
                };
                let mut new_path = vec![c];
                new_path.extend(&path);
                stack.push((nx, ny, new_path));
            }
        }
    }

    cache.insert((start, goal), paths.clone());
    paths
}


pub fn part_two(input: &str) -> Option<u64> {
    let lines = get_lines(input);

    let mut cache = HashMap::new();
    let mut path_cache = HashMap::new();
    let mut result: u64 = 0;
    for line in lines {
        let sequence = line.as_bytes();
        let temp = find_shortest_sequence(sequence, 25, true, &mut cache, &mut path_cache);
        let number: u64 = line[..3].parse().unwrap();
        result += temp as u64 * number;
    }
    Some(result )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
