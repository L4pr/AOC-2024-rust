use advent_of_code::get_lines;

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = get_lines(input);

    let mut result = 0;

    let mut index = 0;
    loop {
        if index >= lines.len() {
            break;
        }
        let button_a = lines[index].split("Button A: ").into_iter().last().unwrap().split(", ").into_iter().map(|x| x.split("+").into_iter().last().unwrap().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let button_b = lines[index + 1].split("Button B: ").into_iter().last().unwrap().split(", ").into_iter().map(|x| x.split("+").into_iter().last().unwrap().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let goal = lines[index + 2].split("Prize:  ").into_iter().last().unwrap().split(", ").into_iter().map(|x| x.split("=").into_iter().last().unwrap().parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let lowest_cost = find_prize(0, 0, button_a[0], button_a[1], button_b[0], button_b[1], 0, 0, goal[0], goal[1]);
        if lowest_cost != -1 {
            result += lowest_cost;
        }
        index += 4;
    }

    Some(result as u32)
}

fn find_prize(x: i32, y: i32, ax: i32, ay: i32, bx: i32, by: i32, a_pressed: u32, b_pressed: u32, goal_x: i32, goal_y: i32) -> i32 {
    if x == goal_x && y == goal_y {
        return 0;
    }
    if x > goal_x || y > goal_y {
        return -1;
    }
    if a_pressed >= 100 {
        return -1;
    }
    let mut b_amount = -1;
    if (goal_x - x) % bx == 0 && (goal_y - y) % by == 0 {
        let temp_amount = (goal_x - x) / bx;
        if temp_amount == (goal_y - y) / by {
            if temp_amount < 100 {
                b_amount = temp_amount;
            }
        }
    }
    let mut newxa = -1;
    if b_pressed < 100 {
        newxa = find_prize(x + ax, y + ay, ax, ay, bx, by, a_pressed + 1, b_pressed, goal_x, goal_y);
    }
    if newxa != -1 {
        if b_amount != -1 {
            if newxa + 3 < b_amount {
                return newxa + 3;
            } else {
                return b_amount;
            }
        }
        return newxa + 3;
    }
    if b_amount != -1 {
        return b_amount;
    }
    return -1;
}

fn find_prize_part2(ax: i64, ay: i64, bx: i64, by: i64, goal_x: i64, goal_y: i64) -> i64 {
    let det_d = ax * by - ay * bx;
    if det_d == 0 {
        return -1;
    }

    let det_da = goal_x * by - goal_y * bx;
    let det_db = ax * goal_y - ay * goal_x;


    if det_da % det_d != 0 || det_db % det_d != 0 {
        return -1;
    }

    let a = det_da / det_d;
    let b = det_db / det_d;

    if a < 0 || b < 0 {
        return -1;
    }

    3 * a + b
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = get_lines(input);

    let mut result = 0;

    let mut index = 0;
    loop {
        if index >= lines.len() {
            break;
        }
        let button_a = lines[index].split("Button A: ").into_iter().last().unwrap().split(", ").into_iter().map(|x| x.split("+").into_iter().last().unwrap().parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let button_b = lines[index + 1].split("Button B: ").into_iter().last().unwrap().split(", ").into_iter().map(|x| x.split("+").into_iter().last().unwrap().parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let goal = lines[index + 2].split("Prize:  ").into_iter().last().unwrap().split(", ").into_iter().map(|x| x.split("=").into_iter().last().unwrap().parse::<i64>().unwrap()).collect::<Vec<i64>>();
        let lowest_cost = find_prize_part2(button_a[0], button_a[1], button_b[0], button_b[1], goal[0] + 10000000000000, goal[1] + 10000000000000);
        if lowest_cost != -1 {
            result += lowest_cost;
        }
        index += 4;
    }

    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
