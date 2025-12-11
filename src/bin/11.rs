use std::{cell::RefCell, collections::HashMap, hash::Hash, rc::Rc};

advent_of_code::solution!(11);

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut lookup = HashMap::new();
    for line in input.lines() {
        let from = &line[0..3];
        let to = line[5..].split(' ').collect::<Vec<_>>();
        lookup.insert(from, to);
    }
    lookup
}

pub fn part_one(input: &str) -> Option<u64> {
    fn solve_recursive<'a>(
        node: &'a str,
        nodes: &HashMap<&str, Vec<&'a str>>,
        memo: &mut HashMap<&'a str, u64>,
    ) -> u64 {
        if node == "out" {
            return 1;
        }
        if let Some(&res) = memo.get(node) {
            return res;
        }
        let res = nodes
            .get(node)
            .unwrap()
            .iter()
            .map(|child| solve_recursive(child, nodes, memo))
            .sum();
        memo.insert(node, res);
        res
    }
    let nodes = parse(input);
    let mut memo = HashMap::new();
    solve_recursive("you", &nodes, &mut memo).into()
}

pub fn part_two(input: &str) -> Option<u64> {
    fn solve_recursive<'a>(
        node: &'a str,
        nodes: &HashMap<&str, Vec<&'a str>>,
        memo: &mut HashMap<(&'a str, bool, bool), u64>,
        visited_dac: bool,
        visited_fft: bool,
    ) -> u64 {
        if node == "out" {
            return (visited_dac && visited_fft) as u64;
        }

        let key = (node, visited_dac, visited_fft);
        if let Some(&res) = memo.get(&key) {
            return res;
        }

        let res = nodes
            .get(node)
            .unwrap()
            .iter()
            .map(|child| {
                solve_recursive(
                    child,
                    nodes,
                    memo,
                    visited_dac || *child == "dac",
                    visited_fft || *child == "fft",
                )
            })
            .sum();
        memo.insert(key, res);
        res
    }

    let nodes = parse(input);
    let mut memo = HashMap::new();
    solve_recursive("svr", &nodes, &mut memo, false, false).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
