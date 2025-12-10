use std::collections::{HashSet, VecDeque};
use z3::{Optimize, ast::Int};

advent_of_code::solution!(10);

#[derive(Debug)]
struct Machine {
    lights: u32,
    buttons: Vec<u32>,
    joltages: Vec<u32>,
}

fn parse(input: &str) -> Vec<Machine> {
    input
        .lines()
        .map(|line| {
            // Parse lights
            let bytes = line.as_bytes();
            let lights_end = bytes.iter().position(|&c| c == b']').unwrap();
            let lights = bytes[1..lights_end]
                .iter()
                .fold(0, |acc, c| (acc << 1) | if *c == b'#' { 1 } else { 0 });
            let n_lights = (lights_end - 1) as u32;

            // Parse buttons
            let mut pos = lights_end + 2;
            let mut buttons = Vec::new();
            let mut cur_buttons = 0;
            while bytes[pos] != b'{' {
                if bytes[pos].is_ascii_digit() {
                    cur_buttons |= 1 << (n_lights - 1 - (bytes[pos] - b'0') as u32);
                } else if bytes[pos] == b')' {
                    buttons.push(cur_buttons);
                    cur_buttons = 0;
                }
                pos += 1;
            }

            // Parse joltages
            let joltages = bytes[pos + 1..bytes.len() - 1]
                .split(|&c| c == b',')
                .map(|num_bytes| {
                    let num_str = std::str::from_utf8(num_bytes).unwrap().trim();
                    num_str.parse::<u32>().unwrap()
                })
                .collect::<Vec<u32>>();

            Machine {
                lights,
                buttons,
                joltages,
            }
        })
        .collect()
}

fn min_presses_for_lights(machine: &Machine) -> u64 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back((0u32, 0u64)); // (current_lights, presses)
    visited.insert(0u32);

    while let Some((current_lights, presses)) = queue.pop_front() {
        if current_lights == machine.lights {
            return presses;
        }
        for &button in &machine.buttons {
            let new_lights = current_lights ^ button;
            if !visited.contains(&new_lights) {
                visited.insert(new_lights);
                queue.push_back((new_lights, presses + 1));
            }
        }
    }

    unreachable!("No solution found")
}

fn min_presses_for_joltages(machine: &Machine) -> u64 {
    let opt = Optimize::new();

    // Number of times each button is pressed
    let press_counts = (0..machine.buttons.len() as u32)
        .map(|i| Int::new_const(format!("button_{}_pressed", i)))
        .collect::<Vec<_>>();

    // Constraint: each button is pressed a non-negative number of times
    for count in &press_counts {
        opt.assert(&count.ge(&Int::from_i64(0)));
    }

    // Constraint: the sum of button presses that affect each joltage must equal the target joltage
    let n_joltages = machine.joltages.len();
    for joltage_idx in 0..n_joltages {
        let sum = Int::add(
            &machine
                .buttons
                .iter()
                .enumerate()
                .filter_map(|(button_idx, button)| {
                    if (button & (1 << (n_joltages as u32 - 1 - joltage_idx as u32))) != 0 {
                        Some(&press_counts[button_idx])
                    } else {
                        None
                    }
                })
                .collect::<Vec<&Int>>(),
        );
        let target = Int::from_u64(machine.joltages[joltage_idx] as u64);
        opt.assert(&sum.eq(&target));
    }

    // Objective: minimize total button presses
    let presses = Int::add(
        &press_counts
            .iter()
            .map(|button_var| button_var.clone())
            .collect::<Vec<Int>>(),
    );
    opt.minimize(&presses);
    if opt.check(&[]) != z3::SatResult::Sat {
        panic!();
    }
    let model = opt.get_model().unwrap();
    press_counts
        .iter()
        .map(|count| model.eval(count, true).unwrap().as_u64().unwrap())
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let machines = parse(input);
    let res = machines
        .iter()
        .map(|machine| min_presses_for_lights(machine))
        .sum();
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let machines = parse(input);
    let res = machines
        .iter()
        .map(|machine| min_presses_for_joltages(machine))
        .sum();
    Some(res)
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
        assert_eq!(result, Some(33));
    }
}
