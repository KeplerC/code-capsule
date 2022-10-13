use utils::error::Result;
extern crate libloading as lib;

fn call_worker_module() -> Result<bool> {
    let lib = lib::Library::new("/home/gdpmobile8/codecapsule/example_lib/lib_worker.so")?;
    unsafe {
        let func: lib::Symbol<unsafe extern fn() -> u32> = lib.get(b"run_worker")?;
        let ret = func();
        match ret {
            0 => Ok(true),
            _ => Ok(false)
        }
    }
}

/// Return, randomly, true or false
pub fn run_worker() -> Result<bool> {
    call_worker_module()
}
