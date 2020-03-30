use opengl_graphics::GlGraphics;
use std::collections::LinkedList;
use piston::input::RenderArgs;
use graphics::types::Color;

#[derive(Clone, PartialEq)]
pub(crate) enum Direction {
    RIGHT,
    LEFT,
    UP,
    DOWN,
}

#[derive(Clone)]
pub(crate) struct SnakePiece(pub u32, pub u32);

pub struct Snake {
    pub(crate)  gl: GlGraphics,
    pub(crate)  snake_parts: LinkedList<SnakePiece>,
    pub(crate)  dir: Direction,
    pub(crate)  width: u32,
}

impl Snake {
    pub(crate) fn render(&mut self, args: &RenderArgs) {
        const SNAKE_COLOR: Color = [0.756, 0.749, 0.062, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.snake_parts
            .iter()
            .map(|piece| SnakePiece(piece.0 * self.width, piece.1 * self.width))
            .map(|piece| graphics::rectangle::square(piece.0 as f64, piece.1 as f64, self.width as f64)
            ).collect();

        self.gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;
            squares
                .into_iter()
                .for_each(|square| graphics::rectangle(SNAKE_COLOR, square, transform, gl));
        })
    }

    pub(crate) fn update(&mut self, just_eaten: bool, cols: u32, rows: u32) -> bool {
        let mut new_head: SnakePiece =
            (*self.snake_parts.front().expect("Snake has no body")).clone();

        if (self.dir == Direction::UP && new_head.1 == 0)
            || (self.dir == Direction::LEFT && new_head.0 == 0)
            || (self.dir == Direction::DOWN && new_head.1 == rows - 1)
            || (self.dir == Direction::RIGHT && new_head.0 == cols - 1)
        {
            println!("Don't eat the wall!");
            return false;
        }

        match self.dir {
            Direction::UP => new_head.1 -= 1,
            Direction::DOWN => new_head.1 += 1,
            Direction::LEFT => new_head.0 -= 1,
            Direction::RIGHT => new_head.0 += 1,
        }

        if !just_eaten {
            self.snake_parts.pop_back();
        }

        // Checks self collision.
        if self.is_collide(new_head.0, new_head.1) {
            println!("Don't eat yourself!");
            return false;
        }

        self.snake_parts.push_front(new_head);
        true
    }

    pub(crate) fn is_collide(&self, x: u32, y: u32) -> bool {
        self.snake_parts.iter().any(|piece| piece.0 == x && piece.1 == y)
    }
}
