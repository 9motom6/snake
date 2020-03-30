extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use std::collections::LinkedList;
use std::iter::FromIterator;

use glutin_window::{GlutinWindow as Window};
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventLoop, Events, EventSettings};
use piston::input::{ButtonEvent, ButtonState, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

mod snake;
mod game;
mod food;

const COLS: u32 = 30;
const ROWS: u32 = 20;
const SQUARE_WIDTH: u32 = 20;

const WIDTH: u32 = COLS * SQUARE_WIDTH;
const HEIGHT: u32 = ROWS * SQUARE_WIDTH;


fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let window: Window = WindowSettings::new("Snake", [WIDTH, HEIGHT])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .unwrap();

    let mut game = game::Game {
        gl: GlGraphics::new(opengl),
        rows: ROWS,
        cols: COLS,
        square_width: SQUARE_WIDTH,
        just_eaten: false,
        score: 0,
        food: food::Food{ x: 5, y: 5},
        snake: snake::Snake {
            gl: GlGraphics::new(opengl),
            snake_parts: LinkedList::from_iter((vec![snake::SnakePiece(COLS/2, ROWS/2)]).into_iter()),
            width: SQUARE_WIDTH,
            dir: snake::Direction::RIGHT,
        },
    };

    run_game(window, &mut game);

    println!("Congratulations, your score was: {}", game.score);
}

fn run_game(mut window: Window, game: &mut game::Game) {
    let mut events = Events::new(EventSettings::new()).ups(9);
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
}