use anyhow::Result;
use wasmtime::{Caller, Engine, Func, Instance, Module, Store};

struct MyState {
    name: String,
    count: usize,
}

fn main() -> Result<()> {
    println!("compiling module");

    let engine = Engine::default();
    let module = Module::from_file(&engine, "examples/hello.wat")?;

    println!("Initializing...");

    let mut store = Store::new(
        &engine,
        MyState {
            name: "hello world".to_string(),
            count: 0,
        },
    );

    println!("Creating callback...");

    let hello_func = Func::wrap(&mut store, |mut caller: Caller<'_, MyState>| {
        println!("Calling back...");
        println!("> {}", caller.data().name);
        caller.data_mut().count += 1;
    });

    println!("Instantiating module...");
    let imports = [hello_func.into()];
    let instance = Instance::new(&mut store, &module, &imports)?;

    println!("Extracting export...");
    let run = instance.get_typed_func::<(), (), _>(&mut store, "run")?;

    println!("Calling export...");
    run.call(&mut store, ())?;

    println!("Done.");
    Ok(())
}
