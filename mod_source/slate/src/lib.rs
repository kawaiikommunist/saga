use bevy_ecs::prelude::*;
use std::sync::RwLock;

pub struct StageState {
    pub world: World,
    pub schedule: Schedule,
}

pub static STATE: RwLock<Option<StageState>> = RwLock::new(None);

#[unsafe(no_mangle)]
pub extern "C" fn init() {
    let world = World::new();
    let schedule = Schedule::default();

    let mut state = STATE.write().unwrap();
    *state = Some(StageState { world, schedule });
}

// 2. Main thread entrypoint (called once per frame by the Host)
#[unsafe(no_mangle)]
pub extern "C" fn update() {
    if let Ok(mut state_guard) = STATE.write() {
        if let Some(ref mut state) = *state_guard {
            // This triggers parallel execution across the subworld threads
            state.schedule.run(&mut state.world);
        }
    }
}

// 3. WORKER THREAD ENTRYPOINT (Called by secondary Host threads)
// Secondary host threads enter here to help execute work pushed by `mod_update`
#[unsafe(no_mangle)]
pub extern "C" fn wasm_worker_entry(worker_id: u32) {
    // Register this Wasm worker with Rayon or your Wasm work-stealing pool
    // Workers sleep/park here until `state.schedule.run()` assigns work
}
