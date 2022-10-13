use utils::error::Result;
extern crate libloading as lib;

fn call_dynamic() -> lib::Result<u32> {
    let lib = lib::Library::new("/home/gdpmobile8/codecapsule/example_lib/lib_worker.so")?;
    unsafe {
        let func: lib::Symbol<unsafe extern fn() -> u32> = lib.get(b"run_worker")?;
        Ok(func())
    }
}

/// Return, randomly, true or false
pub fn generate_hazard() -> Result<bool> {
    call_dynamic();
    Ok(rand::random())
}
