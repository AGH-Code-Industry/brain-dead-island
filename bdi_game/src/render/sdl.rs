pub use super::traits::*;

/// Initializes video submodules, create window, handle canvas.
pub struct  RenderBuilderSDL{

}

/// Manages clusters and renders on screen.
pub struct RenderSDL{

}

impl RenderBuilder<RenderSDL> for RenderBuilderSDL{

    fn new() -> Self {
        todo!();
    }

    fn build(self) -> RenderSDL {
        todo!();
    }

    fn window(self, name : &str, width : u32, heigth : u32) -> Self {
       todo!();
    }

}

impl Render for RenderSDL{

    fn create_cluster(&self) -> super::structures::Cluster {
       todo!();
    }

    fn render(&mut self, cluser : &super::structures::Cluster) {
        
    }

}
