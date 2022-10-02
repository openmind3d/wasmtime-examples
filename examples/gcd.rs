use anyhow::Result;
use wasmtime::{Instance, Module, Store};

fn main() -> Result<()> {
    let mut store = Store::<()>::default();
    let module = Module::from_file(store.engine(), "examples/gcd.wat")?;
    let instance = Instance::new(&mut store, &module, &[])?;

    let gcd = instance.get_typed_func::<(i32, i32), i32, _>(&mut store, "gcd")?;

    println!("gcd(6, 27) = {}", gcd.call(&mut store, (6, 27))?);

    Ok(())
}
