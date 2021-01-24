use std::path::PathBuf;
use tearchan::fs::{file_util, read_bytes_from_file};
use tearchan::scene::context::{SceneContext, SceneRenderContext};
use tearchan::scene::factory::SceneFactory;
use tearchan::scene::{Scene, SceneControlFlow};
use winit::event::WindowEvent;

pub struct FileScene {}

impl FileScene {
    pub fn factory() -> SceneFactory {
        |context, _| {
            context.spawner().spawn_local(read_bytes());
            Box::new(FileScene {})
        }
    }
}

impl Scene for FileScene {
    fn update(&mut self, _context: &mut SceneContext, _event: WindowEvent) -> SceneControlFlow {
        SceneControlFlow::None
    }

    fn render(&mut self, _context: &mut SceneRenderContext) -> SceneControlFlow {
        SceneControlFlow::None
    }
}

async fn read_bytes() {
    log::info!("spawn");
    let mut path = PathBuf::new();
    path.push(file_util().assets_path());
    path.push("example.json");
    let spawner = read_bytes_from_file(path).await;
    log::info!("read_bytes: {:?}", spawner);
}
