use bevy::{
    ecs::{schedule::ScheduleLabel, system::ScheduleSystem},
    prelude::*,
};

pub mod ygg;
pub mod ysys;

#[derive(Resource, Deref, DerefMut)]
pub struct Stage {
    pub world: World,
}

impl Stage {
    pub fn new() -> Self {
        Self {
            world: World::new(),
        }
    }

    pub fn run(&mut self, schedule: impl ScheduleLabel) {
        self.world.run_schedule(schedule);
    }

    pub fn add_systems<M>(
        &mut self,
        schedule: impl ScheduleLabel,
        systems: impl IntoScheduleConfigs<ScheduleSystem, M>,
    ) -> &mut Self {
        let mut sched = self.world.get_resource_or_init::<Schedules>();
        sched.add_systems(schedule, systems);
        self
    }

    pub fn cue<T>(&mut self, cue: impl ScheduleLabel) {
        self.world.run_schedule(cue);
    }
}
