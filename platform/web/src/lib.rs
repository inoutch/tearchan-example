use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn launch_from_wasm() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());

    common::launch_app();
}
