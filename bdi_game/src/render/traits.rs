use super::structures::Cluster;

/// Initializes video submodules, create window, handle canvas.
/// Moves all data to the Renderer and destroys itself.
// we can't make RenderBuilder and Render backend agnostic.
pub trait RenderBuilder<T : Render>{

    /// Creates Renderer.
    fn new() -> Self;

    /// Sets window arguments.
    fn window(self, name : &str, width : u32, heigth : u32) -> Self;

    /// Builds window
    fn build(self) -> T;

}

pub trait Render{
    
    /// Creates cluster.
    fn create_cluster(&self) -> Cluster;

    /// Renders cluster.
    // moved here to keep Cluster backend agnostic.
    fn render(&mut self, cluser : &Cluster);

}
