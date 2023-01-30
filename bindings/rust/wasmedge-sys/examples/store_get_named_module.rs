//!
//! To run this example, follow the commands below:
//!
//! ```bash
//! // go into the directory: bindings/rust
//! cargo run -p wasmedge-sys --example store_get_named_module -- --nocapture
//! ```

use wasmedge_sys::{Config, Vm};
use wasmedge_types::wat2wasm;

#[cfg_attr(test, test)]
#[allow(clippy::assertions_on_result_states)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read the wasm bytes
    let wasm_bytes = wat2wasm(
        br#"
        (module
            (export "fib" (func $fib))
            (func $fib (param $n i32) (result i32)
             (if
              (i32.lt_s
               (get_local $n)
               (i32.const 2)
              )
              (return
               (i32.const 1)
              )
             )
             (return
              (i32.add
               (call $fib
                (i32.sub
                 (get_local $n)
                 (i32.const 2)
                )
               )
               (call $fib
                (i32.sub
                 (get_local $n)
                 (i32.const 1)
                )
               )
              )
             )
            )
           )
"#,
    )?;

    // create a Config context
    let mut config = Config::create()?;
    config.bulk_memory_operations(true);
    assert!(config.bulk_memory_operations_enabled());

    // create a Vm context with the given Config
    let mut vm = Vm::create(Some(config))?;

    // register a wasm module from a in-memory wasm bytes.
    vm.register_instance_from_bytes("extern", &wasm_bytes)?;

    // get the store in vm
    let store = vm.store_mut();

    // get the module named "extern"
    let instance = store.module("extern")?;

    assert!(instance.get_func("fib").is_ok());

    Ok(())
}
