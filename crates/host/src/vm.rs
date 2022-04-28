use anyhow::{bail, Result};
use std::collections::HashMap;

use crate::context;
use wasmtime::*;

type STD = HashMap<String, &'static [u8]>;

pub struct VM<'a> {
    pub engine: Engine,
    pub modules: HashMap<&'a str, Module>,
    std: STD,
}

impl<'a> Default for VM<'a> {
    fn default() -> Self {
        let mut config = Config::new();
        config.wasm_module_linking(true);
        config.wasm_multi_memory(true);

        Self {
            engine: Engine::new(&config).unwrap(),
            modules: HashMap::new(),
            std: Self::load_std(),
        }
    }
}

impl<'a> VM<'a> {
    pub fn instantiate_std(
        &self,
        store: &mut Store<context::Context>,
        linker: &mut Linker<context::Context>,
    ) -> Result<()> {
        for (n, module) in self.modules.iter() {
            let instance = linker.instantiate(store.as_context_mut(), &module)?;
            linker.instance(store.as_context_mut(), n, instance)?;
        }

        Ok(())
    }

    pub fn instantiate_guest(
        &mut self,
        store: &mut Store<context::Context>,
        linker: &mut Linker<context::Context>,
        guest: &Module,
    ) -> Result<Instance> {
        let instance = linker.instantiate(store.as_context_mut(), &guest)?;
        Ok(instance)
    }

    pub fn load(&mut self, name: &'a str) -> Result<&mut Self> {
        let bytes = self.std.get(name);

        if bytes.is_none() {
            bail!("STD module not found: {}", name);
        }

        let module = Module::new(&self.engine, &bytes.unwrap())?;
        self.modules.entry(name).or_insert(module);

        Ok(self)
    }

    pub fn compile(&mut self, binary: &'a [u8]) -> Result<Module> {
        Ok(Module::new(&self.engine, binary)?)
    }

    fn load_std() -> STD {
        let mut std = STD::new();
        std.insert("date".into(), include_bytes!("date.wasm"));
        std
    }
}
