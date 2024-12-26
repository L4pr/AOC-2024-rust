use std::collections::HashSet;
use advent_of_code::{get_lines, CollectVec};

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<u32> {
    let mut edges = vec![[false; 676]; 676];


    let lines = get_lines(input);
    for line in lines {
        let parts = line.split("-").collect_vec();
        let edge1_chars = parts[0].chars().collect_vec();
        let edge1 = 26 * to_usize(edge1_chars[0] as u8) + to_usize(edge1_chars[1] as u8);
        let edge2_chars = parts[1].chars().collect_vec();
        let edge2 = 26 * to_usize(edge2_chars[0] as u8) + to_usize(edge2_chars[1] as u8);
        edges[edge1][edge2] = true;
        edges[edge2][edge1] = true;
    }

    let mut set: HashSet<Vec<usize>> = HashSet::new();

    for i in 0..676 {
        find_all_inter_connected(&mut edges, false, 3, i, 677, i, &mut set, Vec::new());
    }


    Some(set.len() as u32)
}

fn find_all_inter_connected(edges: &mut Vec<[bool;676]>, has_t: bool, depth: i32, goal: usize, came_from: usize, current: usize, all_paths: &mut HashSet<Vec<usize>>, mut path: Vec<usize>) {
    if depth == 0 {
        if has_t && current == goal {
            path.sort();
            all_paths.insert(path);
        }
        return;
    }

    let mut new_has_t = has_t;
    if !has_t {
        new_has_t = starts_with_t(current);
    }

    for i in 0..676 {
        if edges[current][i] == true && i != came_from {
            let mut new_path = path.clone();
            new_path.push(i);
            find_all_inter_connected(edges, new_has_t, depth - 1, goal, current, i, all_paths, new_path);
        }
    }
}

fn starts_with_t(number: usize) -> bool {
    let t = to_usize('t' as u8);
    number / 26 == t
}

fn bron_kerbosch(
    graph: &Vec<HashSet<usize>>,
    r: HashSet<usize>,
    p: HashSet<usize>,
    x: HashSet<usize>,
    cliques: &mut Vec<HashSet<usize>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r);
    } else {
        let mut p_clone = p.clone();
        for &v in &p {
            let neighbors = &graph[v];
            let new_r = &r | &HashSet::from([v]);
            let new_p: HashSet<_> = p_clone.intersection(neighbors).copied().collect();
            let new_x: HashSet<_> = x.intersection(neighbors).copied().collect();

            bron_kerbosch(graph, new_r, new_p, new_x, cliques);

            p_clone.remove(&v);
        }
    }
}

fn largest_clique(graph: &Vec<HashSet<usize>>) -> HashSet<usize> {
    let mut cliques = Vec::new();
    let r = HashSet::new();
    let p: HashSet<_> = (0..676).into_iter().collect();
    let x = HashSet::new();

    bron_kerbosch(graph, r, p, x, &mut cliques);

    cliques.into_iter().max_by_key(|clique| clique.len()).unwrap_or_default()
}

pub fn part_two(input: &str) -> Option<String> {
    let mut edges = vec![HashSet::new(); 676];

    let lines = get_lines(input);
    for line in lines {
        let parts = line.split("-").collect_vec();
        let edge1_chars = parts[0].chars().collect_vec();
        let edge1 = 26 * to_usize(edge1_chars[0] as u8) + to_usize(edge1_chars[1] as u8);
        let edge2_chars = parts[1].chars().collect_vec();
        let edge2 = 26 * to_usize(edge2_chars[0] as u8) + to_usize(edge2_chars[1] as u8);
        edges[edge1].insert(edge2);
        edges[edge2].insert(edge1);
    }

    let mut largest = largest_clique(&edges).into_iter().collect_vec();
    largest.sort();

    let mut result = String::from(to_letter(largest[0] / 26));
    result.push(to_letter(largest[0] % 26));
    for i in 1..largest.len() {
        result.push(',');
        result.push(to_letter(largest[i] / 26));
        result.push(to_letter(largest[i] % 26));
    }

    Some(result)
}


fn to_usize(b: u8) -> usize {
    (b - b'a') as usize
}

fn to_letter(b: usize) -> char {
    (b as u8 + b'a') as char
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("co,de,ka,ta".to_string()));
    }
}
