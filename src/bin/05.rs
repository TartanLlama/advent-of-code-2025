use rust_lapper::{Interval, Lapper};

advent_of_code::solution!(5);

fn parse_intervals(input: &str) -> Lapper<u64, u64> {
    let intervals = input
        .lines()
        .take_while(|s| !s.is_empty())
        .map(|s| {
            let mut split = s.split('-');
            Interval {
                start: split.next().unwrap().parse::<u64>().unwrap(),
                stop: split.next().unwrap().parse::<u64>().unwrap() + 1,
                val: 0,
            }
        })
        .collect::<Vec<_>>();
    let mut tree = Lapper::new(intervals);
    tree.merge_overlaps();
    tree
}

pub fn part_one(input: &str) -> Option<u64> {
    let tree = parse_intervals(input);
    let inventory = input.lines().skip_while(|s| !s.is_empty()).skip(1);
    let res = inventory
        .filter(|item| {
            let n = item.parse::<u64>().unwrap();
            tree.find(n, n + 1).next().is_some()
        })
        .count();
    Some(res as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let tree = parse_intervals(input);
    Some(tree.cov() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
