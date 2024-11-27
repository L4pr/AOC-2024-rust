pub mod template;

pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

pub fn get_lines(input: &str) -> Vec<&str> {
    input.lines().collect::<Vec<&str>>()
}

pub fn make_grid(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.lines().collect();
    let grid: Vec<Vec<char>> = lines
        .iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();
    grid
}

pub fn make_grid_int(input: &str) -> Vec<Vec<u32>> {
    let lines: Vec<&str> = input.lines().collect();
    let grid: Vec<Vec<u32>> = lines
        .iter()
        .map(|line| line.chars().map(|ch| ch as u32 - '0' as u32).collect::<Vec<u32>>())
        .collect();
    grid
}

pub fn get_area_polygon(corners: Vec<(i32, i32)>) -> u64 {
    let length = corners.len();
    let mut s1 = 0;
    let mut s2 = 0;
    for i in 0..length {
        let xi = corners[i].0 as i64;
        let yi = corners[i].1 as i64;
        let xi1 = corners[(i + 1) % length].0 as i64;
        let yi1 = corners[(i + 1) % length].1 as i64;
        s1 += xi * yi1;
        s2 += xi1 * yi;
    }
    ((s1 - s2).abs() / 2) as u64
}

// Use this file to add helper functions and additional modules.
