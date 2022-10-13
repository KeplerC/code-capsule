use utils::error::Result;
extern crate libloading as lib;

fn call_dynamic() -> lib::Result<u32> {
    let lib = lib::Library::new("/path/to/liblibrary.so")?;
    unsafe {
        let func: lib::Symbol<unsafe extern fn() -> u32> = lib.get(b"my_func")?;
        Ok(func())
    }
}

/// Return, randomly, true or false
pub fn generate_hazard() -> Result<bool> {
    Ok(rand::random())
}
