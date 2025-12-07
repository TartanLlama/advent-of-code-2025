advent_of_code::solution!(6);

fn monoid_for(op: u8) -> (fn(u64, u64) -> u64, u64) {
    match op {
        b'+' => ((|a, b| a + b) as fn(u64, u64) -> u64, 0),
        b'*' => ((|a, b| a * b) as fn(u64, u64) -> u64, 1),
        _ => unreachable!(),
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let res = lines
        .last()
        .unwrap()
        .iter()
        .enumerate()
        .fold(0, |acc, (i, op)| {
            let (op, identity) = monoid_for(op.as_bytes()[0]);
            let answer = lines[..lines.len() - 1]
                .iter()
                .fold(identity, |acc, line| op(acc, line[i].parse().unwrap()));
            answer + acc
        });
    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let mut problems = Vec::new();
    let ops = lines.last().unwrap();
    let mut i = 0;
    while i < ops.len() {
        let op = ops[i];
        let start = i;
        i += 1;
        while i < ops.len() && ops[i] == b' ' {
            i += 1;
        }
        if i == ops.len() {
            i += 1;
        }
        problems.push((op, (start, i - 1)));
    }

    let res = problems.iter().fold(0, |acc, &(op, (start, end))| {
        let (op, identity) = monoid_for(op);
        acc + (start..end).rev().fold(identity, |acc, idx| {
            let num = lines[..lines.len() - 1].iter().fold(0_u64, |acc, line| {
                if line[idx] == b' ' {
                    acc
                } else {
                    acc * 10 + (line[idx] - b'0') as u64
                }
            });
            op(acc, num)
        })
    });
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263823));
    }
}
