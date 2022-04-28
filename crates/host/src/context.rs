use wasmtime_wasi::{WasiCtx, WasiCtxBuilder};

pub struct Context {
    pub wasi: WasiCtx,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            wasi: WasiCtxBuilder::new().inherit_stdio().build(),
        }
    }
}
