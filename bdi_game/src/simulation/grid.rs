use log::info;

/// Generic hexagonal rectangle grid representation
pub struct Grid<T>
where
    T: Default + Clone,
{
    pub data: Vec<Vec<GridCell<T>>>,
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    pub fn new(side_len: usize) -> Grid<T> {
        let mut data = Vec::with_capacity(side_len * 2);
        let mut row: Vec<GridCell<T>> = Vec::with_capacity(side_len);

        for _ in 0..side_len {
            row.push(GridCell { data: T::default() });
        }

        for _ in 0..side_len * 2 {
            data.push(row.clone());
        }

        info!("Grid created with side length: {}", side_len);
        info!("Row len: {:?}", row.len());
        Grid { data }
    }

    pub fn is_point_valid(&self, point: &GridPoint) -> bool {
        let (x, y) = Self::get_cell_coord_unchecked(point);
        if y < 0 || y >= self.data.len() as i64 {
            return false;
        }
        if x < 0 || x >= self.data[0].len() as i64 {
            return false;
        }
        true
    }

    pub fn get_cell(&self, point: &GridPoint) -> &GridCell<T> {
        let (x, y) = Self::get_cell_coord(point);
        &self.data[y][x]
    }

    pub fn set_cell_data(&mut self, point: &GridPoint, data: T) {
        let (x, y) = Self::get_cell_coord(point);
        self.data[y][x] = GridCell { data };
    }

    fn get_cell_coord_unchecked(point: &GridPoint) -> (i64, i64) {
        let x = (point.i + (point.j + 1) / 2) as i64;
        let y = point.j as i64;
        (x, y)
    }

    fn get_cell_coord(point: &GridPoint) -> (usize, usize) {
        let (x, y) = Self::get_cell_coord_unchecked(point);
        if x < 0 || y < 0 {
            panic!("({}, {}) is not a valid grid coordinate", point.i, point.j)
        }
        (x as usize, y as usize)
    }

    pub fn get_side_len(&self) -> usize {
        self.data[0].len()
    }
}

/// Point on out haxagonal grid conforming to axial reprezentation.
///
/// More information at: https://www.redblobgames.com/grids/hexagons/#coordinates-axial
pub struct GridPoint {
    pub i: i32,
    pub j: i32,
}

impl GridPoint {
    pub fn new(i: i32, j: i32) -> GridPoint {
        GridPoint { i, j }
    }
}

#[derive(Clone)]
pub struct GridCell<T>
where
    T: Clone,
{
    pub data: T,
}
