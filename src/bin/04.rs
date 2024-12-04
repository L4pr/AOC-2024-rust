use advent_of_code::make_grid;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut result = 0;
    let mut grid = make_grid(input);
    grid.push(vec!['.'; grid[0].len()]);
    grid.insert(0, vec!['.'; grid[0].len()]);
    for i in 0..grid.len() {
        grid[i].push('.');
        grid[i].insert(0, '.');
    }
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'X' {
                for k in 0..8 {
                    if direction_is_word((i as i32, j as i32), k, 1, &grid) {
                        result += 1;
                    }
                }
            }
        }
    }
    Some(result)
}

const DIRECTIONS: [(i32, i32); 8] = [
    (0, 1),
    (1, 0),
    (1, 1),
    (-1, 0),
    (0, -1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];
const LETTERS: [char; 4] = ['X', 'M', 'A', 'S'];

fn direction_is_word(cc: (i32, i32), d: usize, cl: usize, grid: &Vec<Vec<char>>) -> bool {
    if cl >= 4 {
        return true;
    }
    if grid[(cc.0 + DIRECTIONS[d].0) as usize][(cc.1 + DIRECTIONS[d].1) as usize] == LETTERS[cl] {
        return direction_is_word(
            (cc.0 + DIRECTIONS[d].0, cc.1 + DIRECTIONS[d].1),
            d,
            cl + 1,
            grid,
        );
    }
    false
}

const MAS_DIRECTIONS: [(i32, i32); 4] = [(1, 1), (-1, -1), (1, -1), (-1, 1)];

pub fn part_two(input: &str) -> Option<u32> {
    let mut result = 0;
    let mut grid = make_grid(input);
    grid.push(vec!['.'; grid[0].len()]);
    grid.insert(0, vec!['.'; grid[0].len()]);
    for i in 0..grid.len() {
        grid[i].push('.');
        grid[i].insert(0, '.');
    }
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'A' {
                if is_xmas((i as i32, j as i32), &grid) {
                    result += 1;
                }
            }
        }
    }
    Some(result)
}

fn is_xmas(cc: (i32, i32), grid: &Vec<Vec<char>>) -> bool {
    if is_mas(cc, 0, grid) && is_mas(cc, 1, grid) {
        return true;
    }
    false
}

fn is_mas(cc: (i32, i32), diagonal: usize, grid: &Vec<Vec<char>>) -> bool {
    if grid[(cc.0 + MAS_DIRECTIONS[diagonal * 2].0) as usize]
        [(cc.1 + MAS_DIRECTIONS[diagonal * 2].1) as usize]
        == 'M'
    {
        if grid[(cc.0 + MAS_DIRECTIONS[diagonal * 2 + 1].0) as usize]
            [(cc.1 + MAS_DIRECTIONS[diagonal * 2 + 1].1) as usize]
            == 'S'
        {
            return true;
        }
    } else if grid[(cc.0 + MAS_DIRECTIONS[diagonal * 2].0) as usize]
        [(cc.1 + MAS_DIRECTIONS[diagonal * 2].1) as usize]
        == 'S'
    {
        if grid[(cc.0 + MAS_DIRECTIONS[diagonal * 2 + 1].0) as usize]
            [(cc.1 + MAS_DIRECTIONS[diagonal * 2 + 1].1) as usize]
            == 'M'
        {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
