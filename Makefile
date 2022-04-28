.PHONY: standalone shared

standalone:
		wasmtime javascript/date-standalone/build/index.wasm --invoke _start

shared:
		cargo run -p host -- javascript/date-shared/dyn.wasm
