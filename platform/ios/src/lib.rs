pub fn launch_from_ios() -> std::os::raw::c_int {
    std::env::set_var("RUST_LOG", "info");
    common::launch_app();
    0
}

#[no_mangle]
pub extern "C" fn main_rs() -> std::os::raw::c_int {
    stop_unwind(launch_from_ios)
}

fn stop_unwind<F: FnOnce() -> T + std::panic::UnwindSafe, T>(f: F) -> T {
    match std::panic::catch_unwind(f) {
        Ok(t) => t,
        Err(_) => {
            println!("Attempt to Unwind out of rust code");
            std::process::abort()
        }
    }
}
