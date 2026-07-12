pub mod cut {
    pub use bevy::prelude::*;
    pub use wasmtime::component::Component as Comp;
}

mod app;
mod game;
mod script;

use cut::*;

// Temp
use wasmtime::component::{Component, Linker};
use wasmtime::*;

wasmtime::component::bindgen!({
   world: "adder",
   path: "../wasm/wit/adder"
});

fn main() {
    let mut conf = Config::new();
    conf.wasm_component_model(true);

    let engine = Engine::new(&conf).unwrap();
    let mut store = Store::new(&engine, ());
    let linker = Linker::new(&engine);

    let component_path = "target/wasm32-wasip2/debug/wasm.wasm";
    let component = Component::from_file(&engine, component_path).unwrap();

    let instance = Adder::instantiate(&mut store, &component, &linker);

    let result = instance
        .unwrap()
        .docs_adder_add()
        .call_add(&mut store, 5, 7)
        .unwrap();
    println!("Result: {}", result);
}
