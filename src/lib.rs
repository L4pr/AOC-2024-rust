use std::cmp::Ordering;
use std::collections::VecDeque;

pub mod template;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn move_in_direction(direction: usize, current_location: &(i32, i32)) -> (i32, i32) {
    (
        current_location.0 + DIRECTIONS[direction].0,
        current_location.1 + DIRECTIONS[direction].1,
    )
}

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
    input.lines().collect_vec()
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
        .map(|line| {
            line.chars()
                .map(|ch| ch as u32 - '0' as u32)
                .collect::<Vec<u32>>()
        })
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

pub trait CollectVec<T> {
    fn collect_vec(self) -> Vec<T>;
}

impl<T, I> CollectVec<T> for I
where
    I: Iterator<Item = T>,
{
    fn collect_vec(self) -> Vec<T> {
        self.collect()
    }
}

pub struct PriorityQueue<T, F>
where
    F: Fn(&T, &T) -> Ordering,
{
    deque: VecDeque<T>,
    comparator: F,
}

impl<T, F> PriorityQueue<T, F>
where
    F: Fn(&T, &T) -> Ordering,
{
    pub fn new(comparator: F) -> Self {
        Self {
            deque: VecDeque::new(),
            comparator,
        }
    }

    pub fn push(&mut self, value: T) {
        let pos = self
            .deque
            .iter()
            .position(|x| (self.comparator)(x, &value) == Ordering::Greater)
            .unwrap_or(self.deque.len());
        self.deque.insert(pos, value);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.deque.pop_front()
    }

    pub fn peek(&self) -> Option<&T> {
        self.deque.front()
    }

    pub fn is_empty(&self) -> bool {
        self.deque.is_empty()
    }
}

// Use this file to add helper functions and additional modules.
