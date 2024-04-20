use image::GrayImage;
use crate::simulation::grid::{Grid, GridPoint};
use crate::terrain_manager::sampler::Sampler2D;

#[derive(Default, Clone)]
pub(crate) struct WorldCellData {
    elevation: i32,
    // ...
}

pub(crate) type WorldGrid = Grid<WorldCellData>;

/*
impl WorldGrid {
    pub fn from_height_map(height_map: GrayImage) -> WorldGrid {
        let mut grid = WorldGrid::new(height_map.width() as usize);
        let mut offset: i32 = 0;
        let map_sampler = Sampler2D::new(height_map.clone(), height_map.width() as usize);

        for r in 0..height_map.height() {
            for q in -offset..height_map.width() as i32 - offset {
                let elevation = map_sampler.sample_hexagonal_axial(q, r as i32);
                let point = GridPoint::new(q, r as i32);
                grid.set_cell_data(&point, WorldCellData { elevation });
            }

            if r % 2 == 0 {
                offset += 1;
            }
        }

        grid
    }
}
*/