fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    common::launch_app();
}
