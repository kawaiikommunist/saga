use bevy::prelude::*;

use crate::stage::Stage;

mod components;
mod content;
mod stage;

fn main() -> AppExit {
    App::new().add_plugins(DefaultPlugins).run()
}
