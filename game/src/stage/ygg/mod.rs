use std::{any::Any, fmt::Debug, hash::Hash, ops::Index, sync::Arc};

use bevy::ecs::entity::Entity;
use slotmap::{self, SlotMap, new_key_type};

new_key_type! {
    struct RootIdx;
    struct BranchIdx;
    struct LeafIdx;
    struct FnIdx;
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

#[derive(Debug, Clone, Hash)]
pub struct FnNode {
    pub instances: Vec<BranchIdx>,
    // pub exe: dyn Fn,
}

pub struct Yggdrasil {
    roots: SlotMap<RootIdx, RootNode>,
    branches: SlotMap<BranchIdx, BranchNode>,
    leaves: SlotMap<LeafIdx, LeafNode>,
    funcs: SlotMap<FnIdx, FnNode>,
}
