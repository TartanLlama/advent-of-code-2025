pub mod template;

use itertools::Itertools;
use std::ops::{Index, IndexMut};

pub struct Grid {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let lines = input.lines().collect::<Vec<_>>();
        assert!(lines.len() > 0);
        Grid {
            width: lines[0].len(),
            height: lines.len(),
            data: lines
                .into_iter()
                .map(|line| line.bytes())
                .flatten()
                .collect(),
        }
    }
    pub fn height(&self) -> usize {
        self.height
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn in_bounds(&self, y: i32, x: i32) -> bool {
        x >= 0 && x < self.width as i32 && y >= 0 && y < self.height as i32
    }
    pub fn all_coords(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.height).cartesian_product(0..self.width)
    }
    pub fn neighbours(&self, y: usize, x: usize) -> impl Iterator<Item = u8> {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter(move |&(a, b)| {
                let xmod = x as i32 + a;
                let ymod = y as i32 + b;
                (a != 0 || b != 0) && self.in_bounds(ymod, xmod)
            })
            .map(move |(a, b)| self[(y as i32 + b) as usize][(x as i32 + a) as usize])
    }
}

impl Index<usize> for Grid {
    type Output = [u8];
    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.width;
        let end = start + self.width;
        &self.data[start..end]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * self.width;
        let end = start + self.width;
        &mut self.data[start..end]
    }
}
