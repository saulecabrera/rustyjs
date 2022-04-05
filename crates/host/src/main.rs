mod context;
mod vm;

use anyhow::{bail, Result};
use wasmtime::*;
use wasmtime_wasi::sync;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        bail!("Exactly one module path must be provided");
    }

    let module = std::fs::read(&args[1])?;

    let mut vm = vm::VM::default();
    let context = context::Context::default();
    let mut store = Store::new(&vm.engine, context);
    let mut linker = Linker::<context::Context>::new(&vm.engine);

    sync::add_to_linker(&mut linker, |c: &mut context::Context| &mut c.wasi)?;

    vm.load("base64")?
        .instantiate_std(&mut store, &mut linker)?;

    let guest = vm.compile(&module)?;
    let instance = vm.instantiate_guest(&mut store, &mut linker, &guest)?;
    let start = instance.get_typed_func::<(), (), _>(&mut store, "_start")?;

    start.call(&mut store, ())?;

    Ok(())
}
