use utils::error::Result;
extern crate libloading as lib;

fn call_worker_module() -> Result<bool> {

    let random_lib_loader: bool = rand::random::<bool>();
    let lib_dir_path: String = "/home/gdpmobile8/codecapsule/example_lib/".to_owned();
    let lib1_name: &str = "lib_worker_1.so";
    let lib2_name: &str = "lib_worker_2.so";
    let dyn_path:String;

    if random_lib_loader {
        dyn_path = lib_dir_path + lib1_name;
    } else {
        dyn_path = lib_dir_path + lib2_name;
    }

    
    let lib = lib::Library::new(dyn_path)?;
    unsafe {
        let func: lib::Symbol<unsafe extern fn() -> u32> = lib.get(b"run_worker")?;
        let ret = func();
        match ret {
            0 => Ok(true),
            _ => {
                log::error!("Error! Unable to execute"); 
                Ok(false)
            }
        }
    }
}

/// Return, randomly, true or false
pub fn run_worker() -> Result<bool> {
    call_worker_module()
}
