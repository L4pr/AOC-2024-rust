use advent_of_code::{get_lines, CollectVec};

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let mut registers: Vec<u64> = Vec::new();
    let lines = get_lines(input);
    for i in 0..3 {
        let line = lines[i];
        registers.push(line.split(": ").into_iter().last().unwrap().parse::<u64>().unwrap());
    }

    let program = lines[4].split(": ").into_iter().last().unwrap().split(",").collect_vec().iter().map(|s| s.parse::<u64>().unwrap()).collect_vec();

    let mut index = 0;
    let mut output = String::new();
    while index < program.len() {
        match program[index] {
            0 => {
                let nominator = registers[0];
                let denominator = if program[index + 1] < 4 { program[index + 1] } else { registers[(program[index + 1] - 4) as usize] };
                registers[0] = nominator / (2u64.pow(denominator as u32));
            }
            1 => {
                registers[1] = registers[1] ^ program[index + 1];
            }
            2 => {
                registers[1] = (if program[index + 1] < 4 { program[index + 1] } else { registers[(program[index + 1] - 4) as usize] }) % 8;
            }
            3 => {
                if registers[0] != 0 {
                    index = program[index + 1] as usize;
                    continue;
                }
            }
            4 => {
                registers[1] = registers[1] ^ registers[2];
            }
            5 => {
                let combo_operand = if program[index + 1] < 4 { program[index + 1] } else { registers[(program[index + 1] - 4) as usize] };
                output = output + "," + (combo_operand % 8).to_string().as_str();
            }
            6 => {
                let nominator = registers[0];
                let denominator = if program[index + 1] < 4 { program[index + 1] } else { registers[(program[index + 1] - 4) as usize] };
                registers[1] = nominator / (2u64.pow(denominator as u32));
            }
            7 => {
                let nominator = registers[0];
                let denominator = if program[index + 1] < 4 { program[index + 1] } else { registers[(program[index + 1] - 4) as usize] };
                registers[2] = nominator / (2u64.pow(denominator as u32));
            }
            _ => {
                println!("Invalid opcode: {}", program[index]);
                break;
            }
        }
        index += 2;
    }
    Some(output[1..].to_string())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut registers: Vec<u64> = Vec::new();
    let lines = get_lines(input);
    for i in 0..3 {
        let line = lines[i];
        registers.push(line.split(": ").into_iter().last().unwrap().parse::<u64>().unwrap());
    }

    let program = lines[4].split(": ").into_iter().last().unwrap().split(",").collect_vec().iter().map(|s| s.parse::<u64>().unwrap()).collect_vec();
    let mut correct_output = 0u64;
    for i in 0..program.len() {
        correct_output = (correct_output << 4) | program[i];
    }


    let mut index;
    let mut output;
    let mut output_length;

    let program_len = program.len();
    let mut right_numbers = program_len - 1;
    let mut i = 1;
    loop {
        registers[0] = i;
        registers[1] = 0;
        registers[2] = 0;
        index = 0;
        output = 0u64;
        output_length = 0;
        while index < program.len() {
            match program[index] {
                0 => {
                    let nominator = registers[0];
                    let operand = program[index + 1];
                    let denominator = if operand < 4 { operand } else { registers[(operand - 4) as usize] };
                    registers[0] = nominator / (2u64.pow(denominator as u32));
                }
                1 => {
                    registers[1] = registers[1] ^ program[index + 1];
                }
                2 => {
                    let operand = program[index + 1];
                    registers[1] = (if operand < 4 { operand } else { registers[(operand - 4) as usize] }) % 8;
                }
                3 => {
                    if registers[0] != 0 {
                        index = program[index + 1] as usize;
                        continue;
                    }
                }
                4 => {
                    registers[1] = registers[1] ^ registers[2];
                }
                5 => {
                    let operand = program[index + 1];
                    let combo_operand = if operand < 4 { operand } else { registers[(operand - 4) as usize] };
                    output = (output << 4) | (combo_operand % 8);
                    output_length += 1;
                }
                6 => {
                    let nominator = registers[0];
                    let operand = program[index + 1];
                    let denominator = if operand < 4 { operand } else { registers[(operand - 4) as usize] };
                    registers[1] = nominator / (2u64.pow(denominator as u32));
                }
                7 => {
                    let nominator = registers[0];
                    let operand = program[index + 1];
                    let denominator = if operand < 4 { operand } else { registers[(operand - 4) as usize] };
                    registers[2] = nominator / (2u64.pow(denominator as u32));
                }
                _ => {
                    println!("Invalid opcode: {}", program[index]);
                    break;
                }
            }
            index += 2;
        }

        let shift = ((program_len - 1) - right_numbers) * 4;
        let extracted_bits1 = (correct_output >> shift) & 0xF;
        let extracted_bits2 = (output >> shift) & 0xF;
        if correct_output == output && output_length == program_len {
            break;
        }

        if right_numbers == 0 {
            let numberright = program_len - right_numbers;
            for j in 0..numberright {
                let shift = j * 4;
                let extracted_bits = (correct_output >> shift) & 0xF;
                let extracted_bits2 = (output >> shift) & 0xF;
                if extracted_bits != extracted_bits2 {
                    right_numbers = program_len - j - 1;
                    i /= 8;
                    break;
                }
            }
        }

        if extracted_bits1 == extracted_bits2 {
            let numberright = program_len - right_numbers - 1;
            let mut numbers_correct = true;
            for j in 0..numberright {
                let shift = j * 4;
                let extracted_bits = (correct_output >> shift) & 0xF;
                let extracted_bits2 = (output >> shift) & 0xF;
                if extracted_bits != extracted_bits2 {
                    right_numbers = program_len - j - 1;
                    i /= 8;
                    numbers_correct = false;
                    break;
                }
            }

            if numbers_correct {
                i *= 8;
                right_numbers -= 1;
            }
            continue;
        }

        i += 1;
    }
    Some(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("5,7,3,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}
