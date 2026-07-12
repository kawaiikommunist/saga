use crate::cut::*;

// Script[DyFn] -> ExeFn {Vec<Req>, Fnc}
#[derive(Resource)]
pub struct Script;

pub struct ExeFn {
    args: Vec<Req>,
}
