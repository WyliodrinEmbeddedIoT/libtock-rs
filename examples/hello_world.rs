// This example just prints "Hello Tock World" to the terminal.
#![no_std]
use core::fmt::Write;
use libtock::result::TockResult;
use cortex_m_semihosting::{hprintln};
use wabt;
use wasmi::{ModuleInstance, ImportsBuilder, NopExternals, RuntimeValue};


#[libtock::main]
async fn main() -> TockResult<()> {
	hprintln!("Am intrat!").unwrap();
	let wasm_binary: Vec<u8> =
        wabt::wat2wasm(
            r#"
            (module
                (func (export "test") (result i32)
                    i32.const 1337
                )
            )
            "#,
        )
        .expect("failed to parse wat");

    // Load wasm binary and prepare it for instantiation.
    let module = wasmi::Module::from_buffer(&wasm_binary)
        .expect("failed to load wasm");

    // Instantiate a module with empty imports and
    // assert that there is no `start` function.
    let instance =
        ModuleInstance::new(
            &module,
            &ImportsBuilder::default()
        )
        .expect("failed to instantiate wasm module")
        .assert_no_start();

    // Finally, invoke the exported function "test" with no parameters
    // and empty external function executor.
    assert_eq!(
        instance.invoke_export(
            "test",
            &[],
            &mut NopExternals,
        ).expect("failed to execute export"),
        Some(RuntimeValue::I32(1337)),
    );
    let drivers = libtock::retrieve_drivers()?;

    let mut console = drivers.console.create_console();

    writeln!(console, "Hello Tock World")?;

    Ok(())
}

// #[libtock::main]
// async fn main() -> TockResult<()> {
//     let drivers = libtock::retrieve_drivers()?;

//     let mut console = drivers.console.create_console();

//     writeln!(console, "Hello Tock World")?;

//     Ok(())
// }
// This example just prints "Hello Tock World" to the terminal.
