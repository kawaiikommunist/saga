use bevy::{ecs::schedule::ScheduleLabel, prelude::*};

use crate::{
    components::{Empire, InEmpire, InProvince, OnTile, Prod, Province, Tile, Yield},
    stage::Stage,
};

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub struct TurnPassed;

pub struct TurnPlug;

impl Plugin for TurnPlug {
    fn build(&self, app: &mut App) {
        let mut stage = app
            .world_mut()
            .get_resource_mut::<Stage>()
            .expect("TurnPlug inserted and Res<Stage> not initialized");
        let mut sched = stage
            .get_resource_mut::<Schedules>()
            .expect("Stage has no Res<Schedules>");
        let mut turn = sched.entry(TurnPassed);

        turn.configure_sets(
            (
                TLP::JoinWorlds,
                TLP::PreCalc,
                TLP::MainCalc,
                TLP::PostCalc,
                TLP::PreBuildTree,
                TLP::BuildTree,
                TLP::PostBuildTree,
                TLP::PreReview,
                TLP::Review,
                TLP::PostReview,
                TLP::Cleanup,
            )
                .chain(),
        );
    }
}

type TLP = TurnLogicPhase;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum TurnLogicPhase {
    JoinWorlds,
    PreCalc,
    MainCalc,
    PostCalc,
    PreBuildTree,
    BuildTree,
    PostBuildTree,
    PreReview,
    Review,
    PostReview,
    Cleanup,
}

pub struct Ysys {}
