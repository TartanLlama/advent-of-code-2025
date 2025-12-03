advent_of_code::solution!(3);

fn parse_ints(s: &str) -> Vec<u64> {
    s.as_bytes()
        .iter()
        .map(|c| (c - b'0') as u64)
        .collect::<Vec<_>>()
}

// Rust's max function returns the *last* max element, but we need the first
// Rather than reversing the list or using a custom comparator, we just implement
// the algorithm from scratch. This also lets us short-circuit on finding a 9,
// because that's the highest value possible.
fn first_max_with_index(range: &[u64], start: usize, end: usize) -> (usize, u64) {
    let mut best: Option<(usize, u64)> = None;
    for (i, &v) in range[start..end].iter().enumerate() {
        let idx = start + i;
        match best {
            None => best = Some((idx, v)),
            Some((_, max_v)) if v > max_v => best = Some((idx, v)),
            _ => {}
        }
        if v == 9 {
            break;
        }
    }
    best.unwrap()
}

// For the nth battery, scan up to the `end - n`th element looking for the max,
// then build up the result.
fn solve(input: &str, n_batteries: usize) -> u64 {
    input
        .lines()
        .map(|line| {
            let ns = parse_ints(line);
            (0..n_batteries)
                .rev()
                .scan(0usize, |start, i| {
                    let (idx, v) = first_max_with_index(&ns, *start, ns.len() - i);
                    *start = idx + 1;
                    Some(v)
                })
                .fold(0u64, |acc, d| acc * 10 + d)
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, 12))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
