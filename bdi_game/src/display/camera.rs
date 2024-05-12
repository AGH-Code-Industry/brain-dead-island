use crate::util::vec2::Vec2;

pub(crate) struct Camera {
    position: Vec2,
    aspect_ratio: f32,
    init_height: f32,
    scale: f32,
}

impl Camera {
    pub(crate) fn new(aspect_ratio: f32, init_height: f32) -> Self {
        Self {
            position: Vec2::new(0.0, 0.0),
            aspect_ratio,
            init_height,
            scale: 1.0,
        }
    }

    pub(crate) fn set_position(&mut self, position: Vec2) {
        self.position = position;
    }

    fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub(crate) fn world_to_screen(&self, world_position: Vec2) -> Vec2 {
        let x_offset = self.position.y - self.init_height / 2.0 * self.scale * self.aspect_ratio;
        let y_offset = self.position.x - self.init_height / 2.0 * self.scale;

        Vec2::new(world_position.x - x_offset, world_position.y - y_offset)
    }

    pub(crate) fn world_to_screen_multiple(&self, world_positions: Vec<Vec2>) -> Vec<Vec2> {
        let x_offset = self.position.y - self.init_height / 2.0 * self.scale * self.aspect_ratio;
        let y_offset = self.position.x - self.init_height / 2.0 * self.scale;

        world_positions
            .iter()
            .map(|world_position| {
                Vec2::new(world_position.x - x_offset, world_position.y - y_offset)
            })
            .collect()
    }
}
