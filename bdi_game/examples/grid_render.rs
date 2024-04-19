use bdi_game::display::{self, sdl::UnitSDLFillType, traits::*};
use sdl2;

struct GridSetup {
    shape: (u32, u32),
    cell_size: u32,
    cell_pos: Vec<(i32, i32)>,
}

impl GridSetup {
    pub fn new(shape: (u32, u32), cell_size: u32) -> Self {
        let cell_pos = Self::calc_pos(shape, cell_size);
        Self {
            shape,
            cell_size,
            cell_pos,
        }
    }

    pub fn get_pos(&self) -> &Vec<(i32, i32)> {
        &self.cell_pos
    }

    fn calc_pos(shape: (u32, u32), cell_size: u32) -> Vec<(i32, i32)> {
        let width = shape.0;
        let height = shape.1;
        let a = (cell_size / 2) as f64;
        let x_offset = 1.5 * a;
        let y_offset = 3.0_f64.sqrt() * a;

        let mut cell_pos: Vec<(i32, i32)> = Vec::new();
        let mut current_pos: (f64, f64) = (0.0, 0.0);

        for i in 0..width {
            for j in 0..height {
                cell_pos.push((current_pos.0.round() as i32, current_pos.1.round() as i32));
                current_pos.1 += y_offset;
            }
            current_pos.0 += x_offset;
            match i % 2 {
                0 => current_pos.1 = y_offset / 2.0,
                1 => current_pos.1 = 0.0,
                _ => (),
            };
        }

        cell_pos
    }
}

fn main() {
    let mut sdl = sdl2::init().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let mut rend = display::sdl::DisplayBuilderSDL::new(&mut sdl)
        .set_display("game", 1920, 1080)
        .build();

    let setup = GridSetup::new((15, 10), 117);
    let mut cells: Vec<display::sdl::UnitSDL> = setup
        .get_pos()
        .iter()
        .map(|x| display::sdl::UnitSDL::new(&x))
        .collect();

    cells.iter_mut().enumerate().for_each(|(i, x)| {
        x.filling = UnitSDLFillType::Color(sdl2::pixels::Color::RGB(
            20 + 5 * (i%3) as u8,
            100 + 3 * (i%3) as u8,
            55 + 1 * (i%3) as u8,
        ))
    });

    loop {
        for cell in &cells{
            rend.direct_draw_polygon(&cell);
        }
        rend.direct_flush();
        match event_pump.poll_event() {
            Some(sdl2::event::Event::Quit { .. }) => break,
            _ => continue,
        }
    }
}
