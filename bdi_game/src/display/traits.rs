use super::structures::Cluster;

/// Initializes video submodules, create window, handle canvas.
/// Moves all data to the Renderer and destroys itself.
// we can't make RenderBuilder and Render backend agnostic.
// it's called display instead of window to not collide with sdl window structures
pub trait DisplayBuilder {
    type Unit<'a>;
    type Display<'a>: Display<Unit<'a> = Self::Unit<'a>>;

    /// Sets window arguments.
    fn set_display(self, name: &str, width: u32, heigth: u32) -> Self;

    /// Builds window
    fn build<'a>(self) -> Self::Display<'a>;
}

pub trait Display {
    type Unit<'a>;

    /// Creates cluster.
    fn create_cluster<'a>(&self) -> Cluster<Self::Unit<'a>>;

    /// Renders cluster.
    // moved here to keep Cluster backend agnostic.
    fn render<'a>(&mut self, cluser: &Cluster<Self::Unit<'a>>);
}
