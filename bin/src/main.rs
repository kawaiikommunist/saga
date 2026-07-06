pub mod cut {
    pub use bevy::prelude::*;
}

mod app;
mod game;

use cut::*;

fn main() -> AppExit {
    App::new().add_plugins(DefaultPlugins).run()
}
