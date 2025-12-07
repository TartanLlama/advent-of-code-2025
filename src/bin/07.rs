use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(7);

#[derive(Debug)]
struct Problem {
    start: usize,
    levels: Vec<HashSet<usize>>,
}
fn parse_problem(input: &str) -> Problem {
    let mut start = 0;
    let mut levels = Vec::new();
    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            start = line.as_bytes().iter().position(|&c| c == b'S').unwrap();
        } else if i % 2 == 0 {
            levels.push(line.as_bytes().iter().positions(|&c| c == b'^').collect());
        }
    }
    Problem { start, levels }
}

pub fn part_one(input: &str) -> Option<u64> {
    let problem = parse_problem(input);
    let mut beams = HashSet::from([problem.start]);
    let mut times_split = 0;
    for level in problem.levels {
        beams = beams
            .iter()
            .flat_map(|beam| {
                if level.contains(beam) {
                    times_split += 1;
                    vec![beam - 1, beam + 1]
                } else {
                    vec![*beam]
                }
            })
            .collect();
    }
    Some(times_split)
}

fn solve_recursive(
    level: usize,
    beam: usize,
    levels: &Vec<HashSet<usize>>,
    solutions: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if level == levels.len() {
        1
    } else if let Some(solution) = solutions.get(&(level, beam)) {
        *solution
    } else {
        let solution = if levels[level].contains(&beam) {
            solve_recursive(level + 1, beam - 1, levels, solutions)
                + solve_recursive(level + 1, beam + 1, levels, solutions)
        } else {
            solve_recursive(level + 1, beam, levels, solutions)
        };
        solutions.insert((level, beam), solution);
        solution
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let problem = parse_problem(input);
    let mut solutions = HashMap::new();
    let res = solve_recursive(0, problem.start, &problem.levels, &mut solutions);
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
