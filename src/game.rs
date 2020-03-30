use opengl_graphics::GlGraphics;
use crate::{snake, food};
use piston::input::{RenderArgs, Button, Key};
use graphics::types::Color;
use crate::food::Food;

pub(crate) struct Game {
    pub(crate) gl: GlGraphics,
    pub(crate) rows: u32,
    pub(crate) cols: u32,
    pub(crate) square_width: u32,
    pub(crate) just_eaten: bool,
    pub(crate) snake: snake::Snake,
    pub(crate) food: food::Food,
    pub(crate) score: u32
}

impl Game {
    pub(crate) fn render(&mut self, args: &RenderArgs) {
        const GREEN: Color = [0.2, 0.537, 0.282, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(GREEN, gl);
        });

        self.snake.render(args);
        self.food.render(&mut self.gl, args, self.square_width)
    }

    pub(crate) fn update(&mut self) -> bool {

        if !self.snake.update(self.just_eaten, self.cols, self.rows) {
            return false;
        }

        if self.just_eaten {
            self.score += 1;
            self.just_eaten = false;
        }

        self.just_eaten = self.food.update(&self.snake);

        if self.just_eaten {
            self.generate_food();
            println!("Yummy");
        }

        true
    }

    fn generate_food(&mut self) {
        use rand::Rng;
        use rand::thread_rng;

        let mut r = thread_rng();

        loop {
            let new_x = r.gen_range(0, self.cols);
            let new_y = r.gen_range(0, self.rows);

            if !self.snake.is_collide(new_x, new_y) {
                self.food = Food {x: new_x, y: new_y};
                break;
            }

        }
    }

    pub(crate) fn pressed(&mut self, button: &Button) {
        let last_dir = self.snake.dir.clone();

        self.snake.dir = match button {
            &Button::Keyboard(Key::Up) | &Button::Keyboard(Key::W)
            if last_dir != snake::Direction::DOWN => snake::Direction::UP,
            &Button::Keyboard(Key::Right) | &Button::Keyboard(Key::D)
            if last_dir != snake::Direction::LEFT => snake::Direction::RIGHT,
            &Button::Keyboard(Key::Left) | &Button::Keyboard(Key::A)
            if last_dir != snake::Direction::RIGHT => snake::Direction::LEFT,
            &Button::Keyboard(Key::Down) | &Button::Keyboard(Key::S)
            if last_dir != snake::Direction::UP => snake::Direction::DOWN,
            _ => last_dir
        };
    }
}