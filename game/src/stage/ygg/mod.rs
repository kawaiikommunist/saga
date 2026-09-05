use std::{
    any::{Any, TypeId},
    fmt::Debug,
    hash::Hash,
    ops::Index,
    sync::Arc,
};

use bevy::ecs::entity::Entity;
use slotmap::{self, SlotMap, new_key_type};

new_key_type! {
    pub struct RootIdx;
    pub struct BranchIdx;
    pub struct LeafIdx;
    pub struct FnIdx;
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
    pub func: FnIdx,
}

#[derive(Debug, Clone, Hash)]
pub struct LeafNode {
    pub input: Vec<Yidx>,
    pub output: Vec<Entity>,
}

#[derive(Debug, Clone, Hash)]
pub struct FnNode {
    pub instances: Vec<BranchIdx>,
    pub input: Vec<TypeId>,
    pub output: Vec<TypeId>,
    pub exe: fn(&[&dyn Any]) -> Vec<Box<dyn Any>>,
}

pub struct Yggdrasil {
    roots: SlotMap<RootIdx, RootNode>,
    branches: SlotMap<BranchIdx, BranchNode>,
    leaves: SlotMap<LeafIdx, LeafNode>,
    funcs: SlotMap<FnIdx, FnNode>,
}
