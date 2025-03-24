use anyhow::Result;
use std::sync::Arc;
use std::thread::{spawn, JoinHandle};
use wasmer::{imports, Instance, Module, Store};

pub struct WasmVm {
    module: Vec<u8>,
    handle: Option<JoinHandle<Result<()>>>,
}

impl WasmVm {
    pub fn new(module: Vec<u8>) -> Self {
        Self {
            module,
            handle: None,
        }
    }

    pub fn start(&mut self) -> Result<()> {
        let wasm_bytes = self.module.clone();
        let mut store = Store::default();
        let module = Module::new(&store, wasm_bytes)?;
        let import_object = imports! {};
        let instance = Instance::new(&mut store, &module, &import_object)?;
        let handle = spawn(move || -> Result<()> {
            let run_func = instance.exports.get_function("entrypoint")?;
            let result = run_func.call(&mut store, &[])?;
            Ok(())
        });
        self.handle = Some(handle);

        Ok(())
    }
}
