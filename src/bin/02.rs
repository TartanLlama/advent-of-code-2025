use factor::factor::factor;
use multimap::MultiMap;

advent_of_code::solution!(2);

// Parse into pairs representing the number ranges
fn parse(input: &str) -> impl Iterator<Item = (u64, u64)> {
    input.split(',').map(|range| {
        let mut ints = range.split('-').map(|s| s.parse().unwrap());
        (ints.next().unwrap(), ints.next().unwrap())
    })
}
// Solve by finding all numbers in the range that match the given predicate and summing them
fn solve<F: Fn(u64) -> bool>(input: &str, predicate: F) -> Option<u64> {
    let res = parse(input)
        .flat_map(|(from, to)| (from..to + 1).filter(|n| predicate(*n)))
        .sum();
    Some(res)
}
fn n_digits(i: u64) -> u32 {
    i.ilog10() + 1
}

// Check for adjacent pairs of numbers using division and modulo to
// check if the first half of the number equals the second
fn is_pair(i: u64) -> bool {
    if (n_digits(i) % 2) == 1 {
        return false;
    }
    let mask = 10_u64.pow(n_digits(i) / 2);
    i / mask == i % mask
}
pub fn part_one(input: &str) -> Option<u64> {
    solve(input, is_pair)
}

// Generate a map from the number of digits in a number to a list of
// the factors of that number of digits and a corresponding mask that can
// be used to check if a number has repetitions.
// For example, for numbers with 6 digits:
//  factor: 1, mask: 111111
//  factor: 2, mask: 10101
//  factor: 3, mask: 1001
// A number `i` can be checked by checking (i % 10^factor) * mask == i
fn generate_mask_table(max_order: usize) -> MultiMap<usize, (u64, u64)> {
    let mut table = MultiMap::new();
    for order in 1..max_order {
        let mut factors = factor(order as i64);
        factors.push(1);
        for factor in factors {
            let mut mask = 0;
            for _ in 0..order / factor as usize {
                mask = mask * 10_u64.pow(factor as u32) + 1;
            }
            table.insert(order, (factor as u64, mask));
        }
    }
    table
}

// Use the precomputed mask table to check all possible pattern sizes for the
// given number.
fn has_repetition(mask_table: &MultiMap<usize, (u64, u64)>, i: u64) -> bool {
    if let Some(masks) = mask_table.get_vec(&(n_digits(i) as usize)) {
        for (factor, mask) in masks {
            if (i % 10_u64.pow(*factor as u32)) * mask == i {
                return true;
            }
        }
    }
    return false;
}

pub fn part_two(input: &str) -> Option<u64> {
    let mask_table = generate_mask_table(20);
    solve(input, |i| has_repetition(&mask_table, i))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
