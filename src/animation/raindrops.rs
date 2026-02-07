use crate::render::TerminalRenderer;
use crossterm::style::Color;
use std::io;

struct Raindrop {
    x: u16,
    y: f32,
    speed: f32,
    character: char,
}

pub struct RaindropSystem {
    drops: Vec<Raindrop>,
    terminal_width: u16,
    terminal_height: u16,
}

impl RaindropSystem {
    pub fn new(terminal_width: u16, terminal_height: u16) -> Self {
        let drop_count = (terminal_width as usize * terminal_height as usize) / 40;
        let mut drops = Vec::with_capacity(drop_count);

        let characters = ['|', '\'', '.', '`'];

        for i in 0..drop_count {
            drops.push(Raindrop {
                x: (i as u16 * 7) % terminal_width,
                y: ((i as f32 * 3.7) % terminal_height as f32),
                speed: 0.3 + ((i % 5) as f32 * 0.1),
                character: characters[i % characters.len()],
            });
        }

        Self {
            drops,
            terminal_width,
            terminal_height,
        }
    }

    pub fn update(&mut self, terminal_width: u16, terminal_height: u16) {
        if self.terminal_width != terminal_width || self.terminal_height != terminal_height {
            *self = Self::new(terminal_width, terminal_height);
            return;
        }

        for drop in &mut self.drops {
            drop.y += drop.speed;

            if drop.y as u16 >= terminal_height {
                drop.y = 0.0;
                drop.x = (drop.x as usize * 13 + 7) as u16 % terminal_width;
            }
        }
    }

    pub fn render(&self, renderer: &mut TerminalRenderer) -> io::Result<()> {
        for drop in &self.drops {
            let y = drop.y as u16;
            if y < self.terminal_height && drop.x < self.terminal_width {
                renderer.render_char(drop.x, y, drop.character, Color::Cyan)?;
            }
        }
        Ok(())
    }
}
