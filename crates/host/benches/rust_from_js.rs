use criterion::{criterion_group, criterion_main, Criterion};
use host::{context, vm};
use wasmtime::*;
use wasmtime_wasi::sync;

fn benchmark(c: &mut Criterion) {
    c.bench_function("calling rust from js", |b| {
        let mut vm = vm::VM::default();
        let guest_module = vm.compile(include_bytes!("base64-consumer.wasm")).unwrap();

        vm.load("base64").unwrap();

        b.iter(|| {
            let context = context::Context::default();
            let mut store = Store::new(&vm.engine, context);
            let mut linker = Linker::<context::Context>::new(&vm.engine);

            sync::add_to_linker(&mut linker, |c: &mut context::Context| &mut c.wasi).unwrap();
            vm.instantiate_std(&mut store, &mut linker).unwrap();
            let instance = vm.instantiate_guest(&mut store, &mut linker, &guest_module).unwrap();
            let start = instance.get_typed_func::<(), (), _>(&mut store, "_start").unwrap();

            start.call(&mut store, ()).unwrap();
        });
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
