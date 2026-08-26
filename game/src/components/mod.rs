use std::{
    ops::{Index, IndexMut},
    range::Range,
};

use bevy::{
    platform::{collections::HashMap, hash::FixedHasher},
    prelude::*,
};
use grid::Grid;
use multimap::MultiMap;

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Actor;

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Shadow(pub Entity);

#[derive(Debug, Component, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[require(Has)]
pub struct Coords {
    a: u32,
    b: u32,
}

#[derive(Debug, Component, Clone, Copy, Hash)]
pub struct TileData {
    pub elev: i16,
}

impl Coords {
    pub fn a(&self) -> u32 {
        self.a
    }

    pub fn b(&self) -> u32 {
        self.b
    }

    pub fn c(&self) -> i32 {
        -(self.a as i32 + self.b as i32)
    }

    pub fn new(a: u32, b: u32) -> Self {
        Self { a, b }
    }

    pub fn init_range(size: Range<u32>) -> Vec<Self> {
        let mut ret = Vec::new();
        for x in size.clone() {
            for y in size {
                ret.push(Self::new(x, y));
            }
        }
        ret
    }

    pub fn init_region(a: Range<u32>, b: Range<u32>) -> Vec<Self> {
        let mut ret = Vec::new();
        for x in a {
            for y in b {
                ret.push(Self::new(x, y));
            }
        }
        ret
    }
}

#[derive(Debug, Component)]
#[relationship(relationship_target = Has)]
pub struct Loc(pub Entity);

#[derive(Debug, Component, Default)]
#[relationship_target(relationship = Loc)]
pub struct Has(Vec<Entity>);

#[derive(Debug, Component)]
#[relationship(relationship_target = Tile)]
pub struct OnTile(pub Entity);

#[derive(Debug, Component, Default)]
#[relationship_target(relationship = OnTile)]
pub struct Tile(Vec<Entity>);

#[derive(Debug, Component)]
#[relationship(relationship_target = Province)]
pub struct InProvince(pub Entity);

#[derive(Debug, Component, Default)]
#[relationship_target(relationship = InProvince)]
pub struct Province(Vec<Entity>);

#[derive(Debug, Component)]
#[relationship(relationship_target = Empire)]
pub struct InEmpire(pub Entity);

#[derive(Debug, Component, Default)]
#[relationship_target(relationship = (InEmpire))]
pub struct Empire(Vec<Entity>);

macro_rules! e_ident {
    ($($i:ident),*) => {
        $(
        #[derive(Debug, Deref, DerefMut, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        struct $i(pub Entity);
        )*
    };
}

e_ident!(Reso, Unit);

#[derive(Debug, Resource)]
pub struct TileMap {
    grid: Grid<Option<(Entity, TileData)>>,
    hash: HashMap<Entity, Coords>,
}

impl TileMap {
    pub fn new(sx: usize, sy: usize) -> Self {
        Self {
            grid: Grid::new(sx, sy),
            hash: HashMap::new(),
        }
    }
}

impl Index<(usize, usize)> for TileMap {
    type Output = Option<(Entity, TileData)>;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.grid[index]
    }
}

impl IndexMut<(usize, usize)> for TileMap {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.grid[index]
    }
}

impl Index<Coords> for TileMap {
    type Output = Option<(Entity, TileData)>;

    fn index(&self, index: Coords) -> &Self::Output {
        &self[(index.a() as usize, index.b() as usize)]
    }
}

impl IndexMut<Coords> for TileMap {
    fn index_mut(&mut self, index: Coords) -> &mut Self::Output {
        &mut self[(index.a() as usize, index.b() as usize)]
    }
}

// pub struct Yggdrasil {
//     yields: MultiMap<Entity, Branch, FixedHasher>,
// }

// pub struct Branch {
//     yds: Yields,
// }

#[derive(Debug, Component)]
pub struct Prod(pub Vec<(Yields, Ysrc)>);

#[derive(Debug)]
pub struct Yields(pub Vec<Yield>);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Yield(pub u32, pub f32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ysrc(pub Option<u64>);
