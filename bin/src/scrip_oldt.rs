use bevy::{platform::collections::HashMap, utils::Parallel};
use enum_map::{Enum, EnumMap};
use std::any::{Any, TypeId};
use wasmtime::{
    Store,
    component::{ComponentExportIndex as Cei, Func as Fnc, Linker, TypedFunc},
};

use crate::cut::*;

pub struct ModState(HashMap<TypeId, Box<dyn Any + Send>>);

impl ModState {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn set<T: 'static + Send + Any>(&mut self, value: T) {
        self.0.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn get<T: 'static + Send + Any>(&self) -> &T {
        self.0
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>())
            .expect("Attempted to run a WASM system, but a required argument type was missing from HostState!")
    }
}

#[derive(Resource)]
pub struct Script {
    pub mods: Vec<Comp>,
    pub lays: Vec<ModLay>,
    pub link: Linker<ModState>,
    pub stores: Parallel<ExeCtx>,
}

pub struct DySig {
    pub fnc: Fnc,
    pub param: Vec<TypeId>,
}

pub struct ModLay(Vec<DySig>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Enum)]
pub enum FnSig {
    YieldCalc,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModFn {
    pub ty: FnSig,
    pub source: ModId,
    pub id: FnId,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModId(u16);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FnId(u32);

pub struct ExeCtx {
    pub store: Store<ModState>,
    pub cache: HashMap<ModId, Vec<Fnc>>,
}
