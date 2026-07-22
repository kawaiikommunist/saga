use std::path::PathBuf;

use crate::cut::*;
use wasmtime::*;

type ErrOut<T = ()> = bevy::prelude::Result<T, Box<dyn std::error::Error>>;

#[derive(Resource)]
pub struct Stage {
    pub engi: Engine,
    pub mem: SharedMemory,
    pub link: Linker<()>,
    pub store: Store<()>,
    pub inst: Instance,
    pub mods: Vec<Mod>,
}

pub struct Mod {
    pub name: String,
    pub path: PathBuf,
    pub store: Store<()>,
    pub inst: Instance,
}

impl Stage {
    /// Create and initialize the Wasm Subworld runtime
    pub fn new(core_wasm_bytes: &[u8]) -> ErrOut<Self> {
        // Engine
        let mut config = Config::new();
        config.wasm_threads(true);
        config.shared_memory(true);
        let engine = Engine::new(&config)?;

        // SharedMemory
        let memory_type = MemoryType::shared(100, 2000);
        let shared_memory = SharedMemory::new(&engine, memory_type)?;

        // Module
        let core_module = Module::new(&engine, core_wasm_bytes)?;

        // Store
        let mut main_store = Store::new(&engine, ());

        // Linker
        let mut linker = Linker::new(&engine);
        linker.define(&mut main_store, "env", "memory", shared_memory.clone())?;

        // Instance
        let main_instance = linker.instantiate(&mut main_store, &core_module)?;

        // Run Stage.init
        if let Ok(init_fn) = main_instance.get_typed_func::<(), ()>(&mut main_store, "init") {
            init_fn.call(&mut main_store, ())?;
        }

        Ok(Self {
            engi: engine,
            mem: shared_memory,
            link: linker,
            store: main_store,
            inst: main_instance,
            mods: Vec::new(),
        })
    }

    /// Dynamically load and register a third-party mod file at runtime
    pub fn load_mod(&mut self, mod_path: PathBuf) -> ErrOut {
        // Setup
        let mod_bytes = std::fs::read(&mod_path)?;
        let mod_module = Module::new(&self.engi, &mod_bytes)?;
        let mut mod_store = Store::new(&self.engi, ());
        let mod_instance = self.link.instantiate(&mut mod_store, &mod_module)?;

        if let Ok(register_fn) =
            mod_instance.get_typed_func::<(), ()>(&mut mod_store, "register")
        {
            register_fn.call(&mut mod_store, ())?;
        }

        self.mods.push(Mod {
            name: mod_path.file_name().unwrap().to_string_lossy().to_string(),
            path: mod_path,
            store: mod_store,
            inst: mod_instance,
        });

        Ok(())
    }

    /// Tick the Wasm subworld logic loop
    pub fn update(&mut self, delta_seconds: f32) -> ErrOut {
        let update_fn = self
            .inst
            .get_typed_func::<f32, ()>(&mut self.store, "update")?;

        update_fn.call(&mut self.store, delta_seconds)?;
        Ok(())
    }
}
