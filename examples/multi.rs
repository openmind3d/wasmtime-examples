use anyhow::Result;
use wasmtime::{Engine, Func, Instance, Module, Store};

fn main() -> Result<()> {
    println!("Initializing...");
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());

    println!("Compiling module...");
    let module = Module::from_file(&engine, "examples/multi.wat")?;

    println!("Creating callback...");
    let callback_func = Func::wrap(&mut store, |a: i32, b: i64| -> (i64, i32) {
        (b + 1, a + 1)
    });

    println!("Instantiating module...");
    let instance = Instance::new(&mut store, &module, &[callback_func.into()])?;

    println!("Extracting export...");
    let g = instance.get_typed_func::<(i32, i64), (i64, i32), _>(&mut store, "g")?;

    println!("Calling export \"g\"...");
    let (a, b) = g.call(&mut store, (1_i32, 3_i64))?;

    println!("Printing result...");
    println!("> {} {}", a, b);

    assert_eq!(a, 4_i64);
    assert_eq!(b, 2_i32);

    println!("Calling export \"round_trip_many\"...");
    let round_trip_many = instance.get_typed_func::<  
        (i64, i64, i64, i64, i64, i64, i64, i64, i64, i64),
        (i64, i64, i64, i64, i64, i64, i64, i64, i64, i64),
        _,
        >(&mut store, "round_trip_many")?;

    let results = round_trip_many.call(&mut store, (0, 1, 2, 3, 4, 5, 6, 7, 8, 9))?;

    println!("Pringing result...");
    println!("> {:?}", results);
    assert_eq!(results, (0, 1, 2, 3, 4, 5, 6, 7, 8, 9));
    
    Ok(())
}
