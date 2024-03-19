/// Initializes video submodules, create window, handle canvas.
pub trait Render{

    /// Creates Renderer.
    /// Also creates window.
    fn new() -> Self;

}

/// Groups objects to be rendered. 
pub trait Cluster{

    /// Adds units to the cluster.
    fn add();
    /// Renders whole cluser.
    fn render();

}

/// Describes properties of the rendered object.
pub trait Unit{
    fn new();

}
