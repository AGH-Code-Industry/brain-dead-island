pub use super::traits::*;
use sdl2;

/// SDL unit
pub struct UnitSDL<'a>{
    pub position : &'a (i32,i32),
    pub texture : Option<&'a sdl2::render::Texture<'a>>,
    pub color : Option<sdl2::pixels::Color>,
}

impl <'a>UnitSDL<'a>{
    pub fn new(position : &'a (i32,i32)) -> Self{

        Self{
            position,
            texture : None,
            color :None
        }
    }
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
    tile : sdl2::rect::Rect,

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
        let tile = sdl2::rect::Rect::new(0,0,100,100);
        RenderSDL{
            builder : self,
            canvas,
            tile,
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

    fn render<'a>(&mut self, cluster : &super::structures::Cluster<Self::Unit<'a>>) {

        self.canvas.set_draw_color(sdl2::pixels::Color::WHITE);
        for unit in &cluster.objects{

            self.tile.set_x(unit.position.0);
            self.tile.set_y(unit.position.1);
            if let Some(color) = unit.color{
                self.canvas.set_draw_color(color);
            }
            // We should CATCH this error.
            // Add handling here.
            self.canvas.fill_rect(self.tile).unwrap();

        } 

        self.canvas.present();
        
    }

}
