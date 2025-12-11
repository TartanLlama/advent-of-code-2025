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

fn solve<State>(
    input: &str,
    start_node: &str,
    initial_state: State,
    state_fn: impl Fn(State, &str) -> State,
    path_validator: impl Fn(&State) -> bool,
) -> u64
where
    State: Hash + Eq + Copy,
{
    let nodes = parse(input);
    let mut memo = HashMap::new();
    fn solve_recursive<'a, State>(
        node: &'a str,
        nodes: &HashMap<&str, Vec<&'a str>>,
        memo: &mut HashMap<(&'a str, State), u64>,
        state: State,
        state_fn: &impl Fn(State, &str) -> State,
        path_validator: &impl Fn(&State) -> bool,
    ) -> u64
    where
        State: Hash + Eq + Copy,
    {
        if node == "out" {
            return path_validator(&state) as u64;
        }
        if let Some(&res) = memo.get(&(node, state)) {
            return res;
        }
        let state = state_fn(state, node);
        let res = nodes
            .get(node)
            .unwrap()
            .iter()
            .map(|child| solve_recursive(child, nodes, memo, state, state_fn, path_validator))
            .sum();
        memo.insert((node, state), res);
        res
    }
    solve_recursive(
        start_node,
        &nodes,
        &mut memo,
        initial_state,
        &state_fn,
        &path_validator,
    )
}

pub fn part_one(input: &str) -> Option<u64> {
    solve(input, "you", (), |_, _| (), |_| true).into()
}

pub fn part_two(input: &str) -> Option<u64> {
    solve(
        input,
        "svr",
        (false, false),
        |(visited_dac, visited_fft), node| match node {
            "dac" => (true, visited_fft),
            "fft" => (visited_dac, true),
            _ => (visited_dac, visited_fft),
        },
        |&(visited_dac, visited_fft)| visited_dac && visited_fft,
    )
    .into()
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
