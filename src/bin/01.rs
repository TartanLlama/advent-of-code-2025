advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let res = input
        .lines()
        .map(|line| {
            let n: i64 = line[1..].parse().unwrap();
            if line.starts_with('L') { -n } else { n }
        })
        .scan(50, |acc, n| {
            *acc = (*acc + n).rem_euclid(100);
            Some(*acc)
        })
        .filter(|n| *n == 0)
        .count();
    Some(res as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let res = input
        .lines()
        .map(|line| {
            let n: i64 = line[1..].parse().unwrap();
            if line.starts_with('L') { -n } else { n }
        })
        .fold((50, 0), |(val, mut wrap_count), n| {
            let res = val + n;
            wrap_count += res.abs() / 100;
            wrap_count += (val > 0 && res <= 0) as i64;
            ((res % 100 + 100) % 100, wrap_count)
        })
        .1;
    Some(res as u64)
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
        assert_eq!(result, Some(6));
    }
}
