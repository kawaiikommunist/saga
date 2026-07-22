use std::collections::BTreeMap;
use std::ops::{Index, IndexMut};

use bevy::platform::collections::HashMap;

use crate::book::*;
use crate::cut::*;

pub struct Yggdrasil {
    pub tree: Vec<Branch>,
    pub map: HashMap<Entity, Graft>,
}

impl Yggdrasil {
    pub fn new() -> Self {
        Self {
            tree: Vec::new(),
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, parent: Graft, id: Entity) -> Graft {
        let idx = Graft(self.tree.len());
        self[parent].node.push(idx);
        self.tree.push(Branch {
            id,
            slf: idx,
            parent: Some(parent),
            node: Vec::new(),
            mods: Vec::new(),
        });
        idx
    }

    pub fn edd(&mut self, parent: Entity, id: Entity) -> Graft {
        let idx = Graft(self.tree.len());
        if let Some(&graft) = self.map.get(&parent) {
            self[graft].node.push(idx);
            self.tree.push(Branch {
                id,
                slf: idx,
                parent: Some(graft),
                node: Vec::new(),
                mods: Vec::new(),
            });
        } else {
            panic!("{:?} not found in Yggdrasil Entity Map", parent);
        };
        idx
    }

    pub fn init(&mut self, id: Entity) -> Graft {
        self.tree.push(Branch {
            id,
            slf: Graft(0),
            parent: None,
            node: Vec::new(),
            mods: Vec::new(),
        });
        Graft(0)
    }
}

impl Index<Graft> for Yggdrasil {
    type Output = Branch;

    fn index(&self, index: Graft) -> &Self::Output {
        &self.tree[index.0]
    }
}

impl IndexMut<Graft> for Yggdrasil {
    fn index_mut(&mut self, index: Graft) -> &mut Self::Output {
        &mut self.tree[index.0]
    }
}

impl Index<Entity> for Yggdrasil {
    type Output = Branch;

    fn index(&self, index: Entity) -> &Self::Output {
        if let Some(graft) = self.map.get(&index) {
            &self.tree[graft.0]
        } else {
            panic!("{:?} not found in Yggdrasil Entity Map", index);
        }
    }
}

impl IndexMut<Entity> for Yggdrasil {
    fn index_mut(&mut self, index: Entity) -> &mut Self::Output {
        if let Some(graft) = self.map.get(&index) {
            &mut self.tree[graft.0]
        } else {
            panic!("{:?} not found in Yggdrasil Entity Map", index);
        }
    }
}

pub struct Branch {
    pub id: Entity,
    pub slf: Graft,
    pub parent: Option<Graft>,
    pub node: Vec<Graft>,
    pub mods: Vec<Leaf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Component)]
pub struct Graft(pub usize);

pub struct Leaf {
    pub source: YieldSource,
    pub amount: Vec<(PgNum, f32)>,
}

pub struct Calc {
    pub sys: fn(&World, &Self, Entity) -> Leaf,
    pub source: YieldSource,
    pub exe: bool,
}

impl Calc {
    pub fn new(sys: fn(&World, &Self, Entity) -> Leaf, source: YieldSource) -> Self {
        Self {
            sys,
            exe: true,
            source,
        }
    }

    pub fn run(&self, world: &World, entity: Entity) -> Leaf {
        (self.sys)(world, self, entity)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum YieldSource {
    Parent,
    Modifier(PgNum),
}

#[derive(Debug, Clone, Copy, Component)]
pub struct Actor;
