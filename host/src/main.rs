use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
use wasmtime_wasi;
use anyhow::{anyhow, Result};
// use rand::prelude::*;
use std::time::{Duration, Instant};

const TUSCANY_CITIES: &'static [&'static [u8]] = &[
    b"AREZZO\0",
    b"FIRENZE\0",
    b"GROSSETO\0",
    b"LIVORNO\0",
    b"LUCCA\0",
    b"MASSA-CARRARA\0",
    b"PISA\0",
    b"PISTOIA\0",
    b"PRATO\0",
    b"SIENA\0",
];

const N: usize = 10 * 1000 * 1000;

struct HostStatus {
    city_index: usize,
    wasm_buffer_ptr: Option<u32>,
    count_pisa: usize,
    wasi_ctx: wasmtime_wasi::WasiCtx,
}

fn main() {
    println!("Starting tuscany cities Wasi test!");
    let r = run();
    println!("{:?}", r);
}

fn run() -> Result<()> {
    let engine = Engine::default();
    let wasi_ctx = WasiCtxBuilder::new()
        .inherit_stdout()
        .inherit_stderr()
        .build();
    let host_status = HostStatus {
	city_index: 0,
	wasm_buffer_ptr: None,
	count_pisa: 0,
	wasi_ctx,
    };
    let mut store = Store::new(&engine, host_status);
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s: &mut HostStatus| &mut s.wasi_ctx)?;

    linker.func_wrap("host", "get_tuscany_city", get_tuscany_city_wasm)?;
    let module = Module::from_file(&engine, "../check_if_pisa_plugin/target/wasm32-wasi/release/check_if_pisa_plugin.wasm")?;
    // let module = Module::from_file(&engine, "../check_if_pisa_plugin/target/wasm32-unknown-unknown/release/check_if_pisa_plugin.wasm")?;
    let instance = linker.instantiate(&mut store, &module)?;
    let check_if_pisa_wasi = instance.get_typed_func::<(), u32>(&mut store, "check_if_pisa")?;

    let store_1 = clone_reference_mut(&mut store);
    // print_type_of(&store);
    let mut f_check_if_pisa_wasi = |index: usize| {
	// println!("Call check if pisa {}", index);
	let res = check_if_pisa_wasi.call(store_1.as_context_mut(), ()).unwrap(); 	
	let context = store_1.data_mut();
	context.city_index = (index + 1) % TUSCANY_CITIES.len();
	context.count_pisa += res as usize;
    };
    // print_type_of(&f_check_if_pisa_wasi);
    let duration = get_elapsed_time(&mut f_check_if_pisa_wasi, N);
    println!("check_pisa_wasi run in {:?}, PISA founded = {}", duration, store.data().count_pisa);

    let mut count_pisa = 0;
    let f_check_if_pisa = |index: usize| {
	count_pisa += check_if_pisa(index);
    };
    let duration = get_elapsed_time(f_check_if_pisa, N);
    println!("check_pisa run in {:?}, PISA founded = {}", duration, count_pisa);
    Ok(())
}

fn check_if_pisa(index: usize) -> u32 {
    let city = get_tuscany_city(index);
    if city == b"PISA\0" {
	return 1;
    }
    0
}
 
fn get_tuscany_city<'a>(index: usize) -> &'a [u8] {
    let city_index = index % TUSCANY_CITIES.len();
    TUSCANY_CITIES[city_index]
}

fn get_tuscany_city_wasm<'a>(mut caller: Caller<'a, HostStatus>) -> Result<u32> {
    let host_status = caller.data();
    let mut wasm_buffer_ptr = host_status.wasm_buffer_ptr;
    let city_index = host_status.city_index;
    if wasm_buffer_ptr.is_none() {
	let wasm_malloc = get_wasm_function::<HostStatus, u32, u32>(&mut caller, "wasm_malloc")?;
	wasm_buffer_ptr = Some(wasm_malloc.call(&mut caller, 20)?);	
    }
    let mem = match caller.get_export("memory") {
        Some(Extern::Memory(mem)) => mem,
        _ => return Err(Trap::BadSignature.into()),
    };
    match mem.write(&mut caller, wasm_buffer_ptr.unwrap() as usize, TUSCANY_CITIES[city_index]) {
        Ok(_x) => {},
        _ => return Err(Trap::BadSignature.into()),
    };
    
    let host_status = caller.data_mut();
    host_status.wasm_buffer_ptr = wasm_buffer_ptr;
    Ok(wasm_buffer_ptr.unwrap())
}

fn get_wasm_function<'a, T, Params: WasmParams, Results: WasmResults>(caller: &mut Caller<'a, T>, f_name: &str) -> Result<TypedFunc<Params, Results>> {
    let f = caller.get_export(f_name)
        .and_then(|f| f.into_func())
	.ok_or_else(|| anyhow!("failed to find function export `{}`", f_name))?;

    let f_typed = f.typed::<Params, Results>(caller);
    f_typed
}

fn get_elapsed_time(mut f: impl FnMut(usize), n_iter: usize) -> Duration {
    let start = Instant::now();
    for i in 0..n_iter {
	f(i);
    }
    let duration = start.elapsed();
    duration
}

fn clone_reference_mut<'a, 'b, T>(t: &'a mut T) -> &'b mut T {
    let t = t as *mut T;
    return unsafe { &mut *t };
}

fn print_type_of<T>(t: &T) {
    println!("{}", std::any::type_name::<T>())
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
