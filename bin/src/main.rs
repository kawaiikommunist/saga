pub mod cut {
    pub use bevy::prelude::*;
}

use cut::*;

mod book;
mod conts;
mod stage;
mod tree;

fn main() -> AppExit {
    use conts::*;

    App::new()
        .add_plugins((DefaultPlugins, MainPlug))
        .add_systems(Startup, (init_map, print_map).chain())
        .run()
}

pub struct MainPlug;

impl Plugin for MainPlug {
    fn build(&self, app: &mut App) {
        app.insert_resource(stage::Stage::new());
    }
}
