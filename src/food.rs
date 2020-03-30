use crate::snake;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use graphics::types::Color;

pub struct Food {
    pub(crate) x: u32,
    pub(crate) y: u32
}

impl Food {
    pub(crate) fn update(&mut self, snake: &snake::Snake) -> bool {
        let front = snake.snake_parts.front().unwrap();
        front.0 == self.x && front.1 == self.y
    }

    pub(crate) fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, width: u32) {
        const FOOD_COLOR: Color =[0.623, 0.133, 0.807, 1.0];

        let x = self.x * width;
        let y = self.y * width;

        let square = graphics::rectangle::square(x as f64, y as f64, width as f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(FOOD_COLOR, square, transform, gl)
        })
    }
}