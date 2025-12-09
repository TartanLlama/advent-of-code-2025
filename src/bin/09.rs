use itertools::Itertools;
use std::iter::once;

advent_of_code::solution!(9);

type Coord = (i64, i64);

fn parse(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect::<Vec<Coord>>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let coords = parse(input);
    let res = coords
        .iter()
        .tuple_combinations()
        .map(|(&a, &b)| area((a, b)))
        .max()
        .unwrap();
    Some(res as u64)
}

fn normalize(a: Coord, b: Coord) -> (Coord, Coord) {
    ((a.0.min(b.0), a.1.min(b.1)), (a.0.max(b.0), a.1.max(b.1)))
}

fn point_in_rect(point: Coord, rect: (Coord, Coord)) -> bool {
    let (x, y) = point;
    let ((rx1, ry1), (rx2, ry2)) = rect;
    x > rx1 && x < rx2 && y > ry1 && y < ry2
}

fn valid_rect(rect: (Coord, Coord), coords: &Vec<Coord>) -> bool {
    coords
        .iter()
        .chain(once(&coords[0]))
        .tuple_windows()
        .all(|(&a, &b)| {
            let (top_left, bottom_right) = normalize(a, b);
            !(point_in_rect(top_left, ((0, rect.0.1), rect.1))
                && point_in_rect(bottom_right, (rect.0, (i64::MAX, rect.1.1))))
                && !(point_in_rect(top_left, ((rect.0.0, 0), rect.1))
                    && point_in_rect(bottom_right, (rect.0, (rect.1.0, i64::MAX))))
        })
}

fn area(rect: (Coord, Coord)) -> i64 {
    ((rect.1.0 - rect.0.0).abs() + 1) * ((rect.1.1 - rect.0.1).abs() + 1)
}

pub fn part_two(input: &str) -> Option<u64> {
    let coords = parse(input);
    let max = coords
        .iter()
        .tuple_combinations()
        .sorted_by(|&(&a, &b), &(&c, &d)| area((a, b)).cmp(&area((c, d))).reverse())
        .find(|&(&a, &b)| valid_rect(normalize(a, b), &coords))
        .map(|(&a, &b)| area((a, b)) as u64)
        .unwrap();

    Some(max as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
