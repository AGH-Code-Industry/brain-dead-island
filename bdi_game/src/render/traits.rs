use super::structures::Cluster;

/// Initializes video submodules, create window, handle canvas.
/// Moves all data to the Renderer and destroys itself.
// we can't make RenderBuilder and Render backend agnostic.
//
// DISCLAIMER
// i have no idea how i cooked it : )
// but it's working
pub trait RenderBuilder{

    type Unit<'a>;
    type Render<'a> : Render<Unit<'a> =  Self::Unit<'a>>;

    /// Creates Renderer.
    fn new() -> Self;

    /// Sets window arguments.
    fn window(self, name : &str, width : u32, heigth : u32) -> Self;

    /// Builds window
    fn build<'a>(self) -> Self::Render<'a>;

}

pub trait Render{

    type Unit<'a>;
    
    /// Creates cluster.
    fn create_cluster<'a>(&self) -> Cluster<Self::Unit<'a>>;

    /// Renders cluster.
    // moved here to keep Cluster backend agnostic.
    fn render<'a>(&mut self, cluser : &Cluster<Self::Unit<'a>>);

}
