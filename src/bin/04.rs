advent_of_code::solution!(4);

use advent_of_code::Grid;

pub fn part_one(input: &str) -> Option<u64> {
    let grid = Grid::new(input);
    let count = grid
        .all_coords()
        .filter(|(x, y)| {
            if grid[*x][*y] == b'.' {
                false
            } else {
                grid.neighbours(*x, *y).filter(|n| *n == b'@').count() < 4
            }
        })
        .count();
    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = Grid::new(input);
    let mut count = 0;
    let coords = grid.all_coords().collect::<Vec<_>>();
    loop {
        let mut changed = false;
        for &(y, x) in coords.iter() {
            if grid[y][x] == b'.' {
                continue;
            }
            if grid.neighbours(y, x).filter(|n| *n == b'@').count() < 4 {
                grid[y][x] = b'.';
                count += 1;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
