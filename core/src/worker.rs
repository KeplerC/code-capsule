use utils::error::Result;
extern crate libloading as lib;

fn load_module(lib_name:&str, dst_lib_path:String) -> Result<String> {

    Ok(dst_lib_path + lib_name)
}


fn call_worker_module(dyn_path:String) -> Result<bool> {

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

    let random_lib_loader: bool = rand::random::<bool>();
    let lib_dir_path: String = "/home/gdpmobile8/codecapsule/example_lib/".to_owned();
    let lib1_name: &str = "lib_worker_1.so";
    let lib2_name: &str = "lib_worker_2.so";
    let dyn_path:String;

    if random_lib_loader {
        dyn_path = load_module(lib1_name, lib_dir_path)?;
    } else {
        dyn_path = load_module(lib2_name, lib_dir_path)?;
    }

    call_worker_module(dyn_path)
    // match {
    //     Ok(v) => , 
    //     Err(e) => Err(e)
    // }
    
}
