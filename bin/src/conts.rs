use std::ops;

use crate::cut::*;
use itertools::Itertools;

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    a: i32,
    b: i32,
}

impl Tile {
    pub fn new(a: i32, b: i32) -> Self {
        Self { a, b }
    }

    pub fn abc(&self) -> (i32, i32, i32) {
        (self.a.clone(), self.b.clone(), (-(self.a + self.b)).clone())
    }

    pub fn ab(&self) -> (i32, i32) {
        (self.a.clone(), self.b.clone())
    }

    pub fn a(&self) -> i32 {
        self.a
    }

    pub fn b(&self) -> i32 {
        self.b
    }

    pub fn c(&self) -> i32 {
        -(self.a + self.b)
    }
}

impl ops::Add for Tile {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
        }
    }
}

impl ops::Add<(i32, i32)> for Tile {
    type Output = Self;

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Self {
            a: self.a + rhs.0,
            b: self.b + rhs.1,
        }
    }
}

impl ops::AddAssign for Tile {
    fn add_assign(&mut self, rhs: Self) {
        self.a += rhs.a;
        self.b += rhs.b;
    }
}

impl ops::AddAssign<(i32, i32)> for Tile {
    fn add_assign(&mut self, rhs: (i32, i32)) {
        self.a += rhs.0;
        self.b += rhs.1;
    }
}

pub struct Region {
    pub tiles: Vec<Tile>,
    pub index: usize,
}

impl Iterator for Region {
    type Item = Tile;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = self.index;
        let ret = self.tiles.get(idx).copied();
        if ret.is_some() {
            self.index += 1;
        };
        ret
    }
}

pub fn init_map(mut com: Commands) {
    let range = -32..32;
    for (a, b) in range.clone().cartesian_product(range) {
        com.spawn(Tile::new(a, b));
    }
}

pub fn print_map(tiles: Query<&Tile>) {
    for tile in tiles {
        println!("{:?}", tile);
    }
}
