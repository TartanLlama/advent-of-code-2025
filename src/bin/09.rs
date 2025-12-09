use std::{
    collections::HashSet,
    iter::{self, once},
};

use itertools::Itertools;

advent_of_code::solution!(9);

fn parse(input: &str) -> Vec<(i64, i64)> {
    input
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
        .collect::<Vec<(i64, i64)>>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let coords = parse(input);
    let res = coords
        .iter()
        .cartesian_product(&coords)
        .map(|((a, b), (c, d))| ((a + 1 - c) * (b + 1 - d)).abs())
        .max()
        .unwrap();
    Some(res as u64)
}

fn normalize(a: (i64, i64), b: (i64, i64)) -> ((i64, i64), (i64, i64)) {
    ((a.0.min(b.0), a.1.min(b.1)), (a.0.max(b.0), a.1.max(b.1)))
}

fn point_in_rect(point: (i64, i64), rect: ((i64, i64), (i64, i64))) -> bool {
    let (x, y) = point;
    let ((rx1, ry1), (rx2, ry2)) = rect;
    x > rx1 && x < rx2 && y > ry1 && y < ry2
}

fn valid_rect(rect: ((i64, i64), (i64, i64)), coords: &Vec<(i64, i64)>) -> bool {
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

pub fn part_two(input: &str) -> Option<u64> {
    let coords = parse(input);
    let mut max = 0;
    for (i, &a) in coords.iter().enumerate() {
        for &b in coords.iter().skip(i + 1) {
            let area = ((a.0 + 1 - b.0) * (a.1 + 1 - b.1)).abs();
            if area > max && valid_rect(normalize(a, b), &coords) {
                max = area;
            }
        }
    }
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
