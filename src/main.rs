extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use std::collections::LinkedList;
use std::iter::FromIterator;

use glutin_window::GlutinWindow as Window;
use graphics::types::Color;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventLoop, Events, EventSettings};
use piston::input::{Button, ButtonEvent, ButtonState, Key, RenderArgs, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;


struct Game {
    gl: GlGraphics,
    rows: u32,
    cols: u32,
    square_width: u32,
    just_eaten: bool,
    snake: Snake,
    food: Food,
    score: u32
}

struct Snake {
    gl: GlGraphics,
    snake_parts: LinkedList<SnakePiece>,
    dir: Direction,
    width: u32,
}

#[derive(Clone)]
struct SnakePiece(u32, u32);

#[derive(Clone, PartialEq)]
enum Direction {
    RIGHT,
    LEFT,
    UP,
    DOWN,
}

impl Snake {
    fn render(&mut self, args: &RenderArgs) {
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

    fn update(&mut self, just_eaten: bool, cols: u32, rows: u32) -> bool {
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

    fn is_collide(&self, x: u32, y: u32) -> bool {
        self.snake_parts.iter().any(|piece| piece.0 == x && piece.1 == y)
    }
}

struct Food {
    x: u32,
    y: u32
}

impl Food {
    fn update(&mut self, snake: &Snake) -> bool {
        let front = snake.snake_parts.front().unwrap();
        front.0 == self.x && front.1 == self.y
    }

    fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs, width: u32) {
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

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        const GREEN: Color = [0.2, 0.537, 0.282, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(GREEN, gl);
        });

        self.snake.render(args);
        self.food.render(&mut self.gl, args, self.square_width)
    }

    fn update(&mut self) -> bool {

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

    fn pressed(&mut self, button: &Button) {
        let last_dir = self.snake.dir.clone();

        self.snake.dir = match button {
            &Button::Keyboard(Key::Up) | &Button::Keyboard(Key::W)
            if last_dir != Direction::DOWN => Direction::UP,
            &Button::Keyboard(Key::Right) | &Button::Keyboard(Key::D)
            if last_dir != Direction::LEFT => Direction::RIGHT,
            &Button::Keyboard(Key::Left) | &Button::Keyboard(Key::A)
            if last_dir != Direction::RIGHT => Direction::LEFT,
            &Button::Keyboard(Key::Down) | &Button::Keyboard(Key::S)
            if last_dir != Direction::UP => Direction::DOWN,
            _ => last_dir
        };
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    const COLS: u32 = 30;
    const ROWS: u32 = 20;
    const SQUARE_WIDTH: u32 = 20;

    const WIDTH: u32 = COLS * SQUARE_WIDTH;
    const HEIGHT: u32 = ROWS * SQUARE_WIDTH;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Snake", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        rows: ROWS,
        cols: COLS,
        square_width: SQUARE_WIDTH,
        just_eaten: false,
        score: 0,
        food: Food{ x: 5, y: 5},
        snake: Snake {
            gl: GlGraphics::new(opengl),
            snake_parts: LinkedList::from_iter((vec![SnakePiece(COLS/2, ROWS/2)]).into_iter()),
            width: SQUARE_WIDTH,
            dir: Direction::RIGHT,
        },
    };

    let mut events = Events::new(EventSettings::new()).ups(10);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(_u) = e.update_args() {
            if !game.update() {
                break;
            }
        }

        if let Some(key) = e.button_args() {
            if key.state == ButtonState::Press {
                game.pressed(&key.button);
            }
        }
    }

    println!("Congratulations, your score was: {}", game.score);
}