use itertools::Itertools;
use std::collections::HashMap;

advent_of_code::solution!(8);

#[derive(Hash, PartialEq, Eq, Clone)]
struct Coord {
    x: i64,
    y: i64,
    z: i64,
}

impl Coord {
    fn new(x: i64, y: i64, z: i64) -> Coord {
        Coord { x, y, z }
    }

    fn distance(a: &Coord, b: &Coord) -> f64 {
        (((a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2)) as f64).sqrt()
    }
}

struct Circuits {
    circuits: HashMap<usize, Vec<Coord>>,
    membership: HashMap<Coord, usize>,
}

impl Circuits {
    fn new(coords: &Vec<Coord>) -> Self {
        let mut circuits = Circuits {
            circuits: HashMap::new(),
            membership: HashMap::new(),
        };
        for (i, coord) in coords.iter().enumerate() {
            circuits.circuits.insert(i, vec![coord.clone()]);
            circuits.membership.insert(coord.clone(), i);
        }
        circuits
    }

    fn join(&mut self, a: Coord, b: Coord) {
        let &into_circuit = self.membership.get(&a).unwrap();
        let &from_circuit = self.membership.get(&b).unwrap();

        if into_circuit == from_circuit {
            return;
        }

        let mut from_coords = self.circuits.remove(&from_circuit).unwrap();
        for coord in &from_coords {
            self.membership.insert(coord.clone(), into_circuit);
        }
        self.circuits
            .get_mut(&into_circuit)
            .unwrap()
            .append(&mut from_coords);
    }
    fn result(&self) -> u64 {
        self.circuits
            .values()
            .map(|circuit| circuit.len())
            .sorted()
            .rev()
            .take(3)
            .fold(1, |acc, x| acc * x as u64)
    }
}

fn parse(input: &str) -> Vec<Coord> {
    input
        .lines()
        .map(|line| {
            let mut split = line.split(',').map(|n| n.parse().unwrap());
            Coord::new(
                split.next().unwrap(),
                split.next().unwrap(),
                split.next().unwrap(),
            )
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let coords = parse(input);
    let mut circuits = Circuits::new(&coords);
    for (i, j) in (0..coords.len())
        .cartesian_product(0..coords.len())
        .filter(|(a, b)| a < b)
        .sorted_by(|&(i, j), &(k, l)| {
            Coord::distance(&coords[i], &coords[j])
                .partial_cmp(&Coord::distance(&coords[k], &coords[l]))
                .unwrap()
        })
        .take(1000)
    {
        circuits.join(coords[i].clone(), coords[j].clone());
    }
    Some(circuits.result())
}

pub fn part_two(input: &str) -> Option<u64> {
    let coords = parse(input);
    let mut circuits = Circuits::new(&coords);
    for (i, j) in (0..coords.len())
        .cartesian_product(0..coords.len())
        .filter(|(a, b)| a < b)
        .sorted_by(|&(i, j), &(k, l)| {
            Coord::distance(&coords[i], &coords[j])
                .partial_cmp(&Coord::distance(&coords[k], &coords[l]))
                .unwrap()
        })
    {
        circuits.join(coords[i].clone(), coords[j].clone());
        if circuits.circuits.len() == 1 {
            return Some((coords[i].x * coords[j].x) as u64);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
