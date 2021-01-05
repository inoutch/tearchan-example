pub mod action;
pub mod scene;

use crate::scene::hello_world_scene::HelloWorldScene;
use tearchan::engine::Engine;
use tearchan::engine_config::EngineStartupConfig;

pub fn launch_app() {
    let startup_config =
        EngineStartupConfig::new_with_title("tearchan example", HelloWorldScene::factory());
    let engine = Engine::new(startup_config);
    engine.run();
}
