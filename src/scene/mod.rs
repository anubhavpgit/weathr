pub mod decorations;
pub mod ground;
pub mod house;

use crate::render::TerminalRenderer;
use std::io;

pub struct WorldScene {
    house: house::House,
    ground: ground::Ground,
    decorations: decorations::Decorations,
    width: u16,
    height: u16,
}

impl WorldScene {
    pub fn new(width: u16, height: u16) -> Self {
        let house = house::House;
        let ground = ground::Ground;
        let decorations = decorations::Decorations::new();

        Self {
            house,
            ground,
            decorations,
            width,
            height,
        }
    }

    pub fn update_size(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }

    pub fn render(&self, renderer: &mut TerminalRenderer) -> io::Result<()> {
        let ground_height = 8;
        let horizon_y = self.height.saturating_sub(ground_height);

        // House position
        let house_width = self.house.width();
        let house_height = self.house.height();
        let house_x = (self.width / 2).saturating_sub(house_width / 2);
        let house_y = horizon_y.saturating_sub(house_height);

        // Door/Path alignment
        let door_offset = self.house.door_offset();
        let path_center = house_x + door_offset;

        // Render Ground
        self.ground
            .render(renderer, self.width, ground_height, horizon_y, path_center)?;

        // Render House
        self.house.render(renderer, house_x, house_y)?;

        // Render Decorations
        self.decorations.render(
            renderer,
            horizon_y,
            house_x,
            house_width,
            path_center,
            self.width,
        )?;

        Ok(())
    }
}
