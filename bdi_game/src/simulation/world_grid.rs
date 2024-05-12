use crate::simulation::grid::{Grid, GridPoint};
use crate::terrain_manager::sampler::Sampler2D;
use image::GrayImage;

#[derive(Default, Clone)]
pub struct WorldCellData {
    pub elevation: i32,
    // ...
}

pub(crate) type WorldGrid = Grid<WorldCellData>;

impl WorldGrid {
    pub fn from_height_map(height_map: GrayImage, side_len: usize) -> WorldGrid {
        let mut grid = WorldGrid::new(side_len);
        let mut offset: i32 = 0;
        let map_sampler = Sampler2D::new(height_map.clone(), side_len);

        for r in 0..2 * side_len as i32 {
            for q in -offset..side_len as i32 - offset {
                let elevation = 255 - map_sampler.sample_hexagonal_axial(q, r);
                let point = GridPoint::new(q, r);
                grid.set_cell_data(&point, WorldCellData { elevation });
            }

            if r % 2 == 0 {
                offset += 1;
            }
        }

        grid
    }
}
