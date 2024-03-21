pub use super::traits::*;
use sdl2;

/// SDL unit
pub struct UnitSDL<'a>{
    position : &'a (u32,u32),
    texture : &'a sdl2::render::Texture<'a>,
}

/// Initializes video submodules, create window, handle canvas.
pub struct  RenderBuilderSDL{

    width : u32,
    heigth : u32,
    name : String,
    context : sdl2::Sdl,
    video : sdl2::VideoSubsystem,


}

/// Manages clusters and renders on screen.
pub struct RenderSDL{

    builder : RenderBuilderSDL,
    canvas : sdl2::render::WindowCanvas,

}

impl RenderBuilder for RenderBuilderSDL{

    type Unit<'a> = UnitSDL<'a> where Self : 'a;
    type Render<'a> = RenderSDL;

    fn new() -> Self {
        // No point in catching this errors
        let context = sdl2::init().expect("Cannot initialize SDL library");
        let video = context.video().expect("Cannot start video subsystem");

        Self{
            width : 0,
            heigth : 0,
            name : String::new(),
            context,
            video,
        }
    }

    fn build<'a>(self) -> Self::Render<'a> {
        let canvas = self.video.window(self.name.as_str(), self.width, self.heigth)
            .build()
            .expect("Cannot create window")
            .into_canvas()
            .build()
            .expect("Cannot create canvas");
        RenderSDL{
            builder : self,
            canvas
        }
    }

    fn window(mut self, name : &str, width : u32, heigth : u32) -> Self {
        self.name = name.to_string();
        self.width = width;
        self.heigth = heigth;
        self
    }

}

impl Render for RenderSDL{

    type Unit<'a> = UnitSDL<'a> where Self : 'a;

    fn create_cluster<'a>(&self) -> super::structures::Cluster<Self::Unit<'a>> {
        super::structures::Cluster::new()
    }

    fn render<'a>(&mut self, cluser : &super::structures::Cluster<Self::Unit<'a>>) {
        
    }

}
