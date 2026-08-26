use std::{any::Any, fmt::Debug, hash::Hash, ops::Index, sync::Arc};

use bevy::ecs::entity::Entity;
use slotmap::{self, Key, SlotMap, new_key_type};

new_key_type! {
    struct RootIdx;
    struct BranchIdx;
    struct LeafIdx;
}

#[derive(Debug, Clone, Hash)]
enum Yidx {
    Root(RootIdx),
    Branch(BranchIdx),
    Leaf(LeafIdx),
}

#[derive(Debug, Clone, Hash)]
pub struct RootNode {
    pub output: Vec<Yidx>,
}

#[derive(Debug, Clone, Hash)]
pub struct BranchNode {
    pub input: Vec<Yidx>,
    pub output: Vec<Yidx>,
}

#[derive(Debug, Clone, Hash)]
pub struct LeafNode {
    pub input: Vec<Yidx>,
    pub output: Vec<Entity>,
}

pub struct Yggdrasil {
    roots: SlotMap<RootIdx, RootNode>,
    branches: SlotMap<BranchIdx, BranchNode>,
    leaves: SlotMap<LeafIdx, LeafNode>,
}
