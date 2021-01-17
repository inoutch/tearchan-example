import("./pkg").then(module => {
    window.sc_internal_wrapper().then(sc_internal => {
        window.sc_internal = sc_internal;
        module.launch_from_wasm();
    });
});
