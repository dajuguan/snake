// #[derive(Debug)]

extern crate find_folder;
extern crate piston_window;

mod draw;
mod game;
mod snake;
use draw::{draw_block, draw_rectangle, to_coord_u32};
use game::*;
use piston_window::types::Color;
use piston_window::*;
use snake::*;
const width: i32 = 15;
const height: i32 = 15;
const BACK_COLOR: Color = [225.0 / 255.0, 207.0 / 255.0, 197.0 / 255.0, 1.0];

fn main() {
    let mut window: PistonWindow =
        WindowSettings::new("Snake", [to_coord_u32(width), to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();
    // let assets = find_folder::Search::ParentsThenKids(3, 3)
    //     .for_folder("assets")
    //     .unwrap();
    // println!("{:?}", assets);
    // let mut glyphs = window
    //     .load_font(assets.join("FiraSans-Regular.ttf"))
    //     .unwrap();
    let mut game = Game::new(width, height);
    while let Some(e) = window.next() {
        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&e, |c, g, _| {
            clear(BACK_COLOR, g);
            game.draw(&c, g);
        });
        e.update(|arg| {
            game.update(arg.dt);
        });
    }
}
