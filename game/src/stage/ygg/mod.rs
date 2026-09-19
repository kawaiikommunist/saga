use std::{
    any::{Any, TypeId},
    error::Error,
    fmt::Debug,
    hash::{Hash, Hasher},
    ops::Index,
    sync::Arc,
};

use bitvec::prelude::*;

use bevy::{
    ecs::{component::Component, entity::Entity},
    platform::collections::HashMap,
    prelude::{Deref, DerefMut},
};
use slotmap::{self, SlotMap, new_key_type};

use crate::stage::ygg::Yidx::Branch;

#[derive(Debug, Deref, DerefMut, Hash, PartialEq, Eq)]
pub struct RootKey(BitVec);

pub struct YgUpdateMsg {
    key: RootKey,
    val: Box<dyn YgVal>,
}

pub trait YgVal: Any {
    fn as_any(&self) -> &dyn Any;
    fn dyn_eq(&self, other: &dyn YgVal) -> bool;
    fn clone_box(&self) -> Box<dyn YgVal>;
}

impl<T: Clone + YgVal + Eq + 'static> YgVal for T {
    fn clone_box(&self) -> Box<dyn YgVal> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn dyn_eq(&self, other: &dyn YgVal) -> bool {
        // Downcast `other` to the concrete type `Self`; return false if types mismatch
        if let Some(other) = (other as &dyn Any).downcast_ref::<Self>() {
            self == other
        } else {
            false
        }
    }
}

new_key_type! {
    pub struct RootIdx;
    pub struct BranchIdx;
    pub struct LeafIdx;
    pub struct FnIdx;
}

#[derive(Debug, Clone, Hash)]
pub enum Yidx {
    Root(RootIdx),
    Branch(BranchIdx),
    Leaf(LeafIdx),
}

pub enum YNode {
    Root(RootNode),
    Branch(BranchNode),
    Leaf(LeafNode),
}

pub struct RootNode {
    // The expected output type
    pub typ: TypeId,
    // Which nodes this outputs to
    pub output: Vec<Yidx>,
    // the cache of the output
    pub cache: Box<dyn YgVal>,
}

pub struct BranchNode {
    pub input: Vec<Yidx>,
    pub output: Vec<Yidx>,
    pub func: FnIdx,
    pub cache: Box<dyn YgVal>,
}

pub struct LeafNode {
    pub input: Vec<Yidx>,
    pub typ: TypeId,
    pub output: Vec<Entity>,
}

#[derive(Debug)]
pub struct FnNode {
    pub instances: Vec<BranchIdx>,
    pub input: Vec<TypeId>,
    pub output: TypeId,
    // Takes in the slice of input data && the address for the cache
    // Returns true if the cache changed and false otherwise
    pub exe: fn(&[&dyn Any], &mut dyn Any) -> bool,
}

pub struct Yggdrasil {
    table: HashMap<BitVec, RootIdx>,
    roots: SlotMap<RootIdx, RootNode>,
    branches: SlotMap<BranchIdx, BranchNode>,
    leaves: SlotMap<LeafIdx, LeafNode>,
    funcs: SlotMap<FnIdx, FnNode>,
}

impl Yggdrasil {
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
            roots: SlotMap::with_key(),
            branches: SlotMap::with_key(),
            leaves: SlotMap::with_key(),
            funcs: SlotMap::with_key(),
        }
    }

    pub fn check_rk(&self, key: &RootKey) -> bool {
        self.table.contains_key(&key.0)
    }

    pub fn get_rk_type<T>(&self, key: &RootKey) -> Option<TypeId> {
        if let Some(root) = self.table.get(&key.0) {
            Some(self.roots[*root].typ)
        } else {
            None
        }
    }

    pub fn update(&mut self, msgs: &[YgUpdateMsg]) -> Result<(), ()> {
        for msg in msgs {
            let root = self.table.get(&msg.key.0).unwrap();
        }

        Ok(())
    }
}

impl Index<Yidx> for Yggdrasil {
    type Output = YNode;

    fn index(&self, index: Yidx) -> &Self::Output {
        match index {
            Root(root) => {}
            Branch(branch) => {}
            Leaf(leaf) => {}
        }
    }
}
