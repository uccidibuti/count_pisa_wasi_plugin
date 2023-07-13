# count_pisa_wasi_plugin
A stupid Rust Wasmtime Wasi host-plugin demo that shows how create and load a wasi plugin which call host functions.
## Getting Started
1) Compile Wasi plugin:
```
(cd check_if_pisa_plugin/;cargo build --release --target wasm32-wasi)
```
2) Compile and run Host program:
```
(cd host/;cargo run --release)
```
## About check_if_pisa_plugin
The plugin exposes 'check_if_pisa' function. This function get a random Tuscany cities calling 'get_tuscany_city' Host function and return 1 if the city is 'Pisa'.
## About Host
Host program loads "check_if_pisa_plugin" wasi plugin passing 'get_tuscany_city' as extern function and compares the performance between calling 'check_if_pisa' plugin function and a native implementation for N = 10M times.
An example of output:
```
Starting tuscany cities Wasi test!
check_pisa_wasi runned in 1.408480067s, PISA founded = 1000000
check_pisa runned in 14.18776ms, PISA founded = 1000000
Ok(())
```

