use wasmtime::ResourceLimiter;
use wasmtime_wasi::{sync, WasiCtx, WasiCtxBuilder};

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
