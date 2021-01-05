#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn launch_from_android() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    common::launch_app();
}
