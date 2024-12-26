use std::collections::HashMap;
use advent_of_code::{get_lines, CollectVec};

advent_of_code::solution!(24);

pub fn part_one(input: &str) -> Option<u64> {
    let mut table: HashMap<&str, u32> = HashMap::new();
    let lines = get_lines(input);
    let mut ports = false;
    let mut queue = Vec::new();
    for line in lines {
        if line.is_empty() {
            ports = true;
            continue;
        }
        if ports {
            queue.push(line);
        } else {
            let parts = line.split(": ").collect_vec();
            table.insert(parts[0], parts[1].parse::<u32>().unwrap());
        }
    }

    while !queue.is_empty() {
        let line = queue.pop().unwrap();
        let parts = line.split(" -> ").collect_vec();
        let inputs = parts[0].split_whitespace().collect_vec();
        if !table.contains_key(inputs[0]) || !table.contains_key(inputs[2]) {
            queue.insert(0, line);
            continue;
        }
        if parts[0].contains("AND") {
            let new_value = table.get(inputs[0]).unwrap() & table.get(inputs[2]).unwrap();
            table.entry(parts[1]).or_insert(new_value);
        } else if parts[0].contains("XOR") {
            let new_value = table.get(inputs[0]).unwrap() ^ table.get(inputs[2]).unwrap();
            table.entry(parts[1]).or_insert(new_value);
        } else {
            let new_value = table.get(inputs[0]).unwrap() | table.get(inputs[2]).unwrap();
            table.entry(parts[1]).or_insert(new_value);
        }
    }

    let mut result = 0;

    for key in table.keys() {
        if key.starts_with('z') {
            let number = table.get(key).unwrap();
            if *number == 1 {
                let shift = key.split("z").collect_vec()[1].parse::<u32>().unwrap();
                result += 1 << shift;
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut table: HashMap<&str, u32> = HashMap::new();
    let lines = get_lines(input);
    let mut ports = false;
    let mut queue = Vec::new();
    let mut x: u64 = 0;
    let mut y: u64 = 0;
    for line in lines {
        if line.is_empty() {
            ports = true;
            continue;
        }
        if ports {
            let parts = line.split(" -> ").collect_vec();
            let inputs = parts[0].split_whitespace().collect_vec();
            queue.push(((inputs[0], inputs[1], inputs[2]), parts[1]));
        } else {
            let parts = line.split(": ").collect_vec();
            if parts[1] == "1" {
                if parts[0].starts_with("x") {
                    let shift = parts[0].split("x").collect_vec()[1].parse::<u32>().unwrap();
                    x += 1 << shift;
                } else {
                    let shift = parts[0].split("y").collect_vec()[1].parse::<u32>().unwrap();
                    y += 1 << shift;
                }
            }
            table.insert(parts[0], parts[1].parse::<u32>().unwrap());
        }
    }

    let goal_z = x + y;

    let mut dot = String::from("digraph Gates {\n");

    for ((input1, gate, input2), output) in queue.clone() {
        // Add inputs to the gate
        dot.push_str(&format!("\"{input1}\" -> \"{gate}_{output}\";\n"));
        dot.push_str(&format!("\"{input2}\" -> \"{gate}_{output}\";\n"));

        // Add gate to the output
        dot.push_str(&format!("\"{gate}_{output}\" -> \"{output}\";\n"));
    }

    dot.push_str("}");

    println!("{}", dot);

    let mut queue = queue.clone();

    while !queue.is_empty() {
        let line = queue.pop().unwrap();
        if !table.contains_key(line.0.0) || !table.contains_key(line.0.2) {
            queue.insert(0, line);
            continue;
        }
        if line.0.1.contains("AND") {
            let new_value = table.get(line.0.0).unwrap() & table.get(line.0.2).unwrap();
            table.entry(line.1).or_insert(new_value);
        } else if line.0.1.contains("XOR") {
            let new_value = table.get(line.0.0).unwrap() ^ table.get(line.0.2).unwrap();
            table.entry(line.1).or_insert(new_value);
        } else {
            let new_value = table.get(line.0.0).unwrap() | table.get(line.0.2).unwrap();
            table.entry(line.1).or_insert(new_value);
        }
    }

    let mut result = 0;

    for key in table.keys() {
        if key.starts_with('z') {
            let number = table.get(key).unwrap();
            if *number == 1 {
                let shift = key.split("z").collect_vec()[1].parse::<u32>().unwrap();
                result += 1 << shift;
            }
        }
    }

    println!("result: {:b}", result);
    println!("goal_z: {:b}", goal_z);
    println!("wrong_:     {:b}", result ^ goal_z);
    println!("indexes:5432109876543210987654321098765432109876543210");
    println!("wrong_bits_amount: {}", (result ^ goal_z).count_ones());

    // right answer is cpm,ghp,gpr,krs,nks,z10,z21,z33
    // found by hand.

    Some(1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
