use std::error::Error;
use wasmtime::{Caller, Engine, Linker, Module, Store};

struct Log {
    integers_logged: Vec<u32>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let engine = Engine::default();
    let module = Module::from_file(&engine, "examples/log.wat")?;

    let mut linker = Linker::new(&engine);

    linker.func_wrap("", "double", |param: i32| param * 2)?;

    linker.func_wrap("", "log", |mut caller: Caller<'_, Log>, param: u32| {
        println!("log: {}", param);
        caller.data_mut().integers_logged.push(param);
    })?;

    let data = Log {
        integers_logged: Vec::new(),
    };

    let mut store = Store::new(&engine, data);
    let instance = linker.instantiate(&mut store, &module)?;

    let run = instance.get_typed_func::<(), (), _>(&mut store, "run")?;
    run.call(&mut store, ())?;

    println!("logged integers: {:?}", store.data().integers_logged);

    Ok(())
}
