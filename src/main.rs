/* snake game has four main element:
    1. snake: snake fails if it hits itself or hit the wall
    2. apple: appears randomly
    3. wall
    4. game board */

extern crate rand;    // importing external library to or project
extern crate piston_window;

mod snake;
mod draw;  // we have another file in our project that need to connect
mod game;

use piston_window::*;
use piston_window::types::Color;

use game::Game;
use draw::to_coord_u32;

const BACK_COLOR: Color = [0.5, 0.5, 0.5, 1.0];

fn main() {
    let (width, height) = (20, 20);

    let mut window: PistonWindow = WindowSettings::new(
        "snake",
        [to_coord_u32(width), to_coord_u32(height)],
    ).exit_on_esc(true).build().unwrap(); //exit the game if esc is pressed, build the window

    let mut game = Game::new(width, height);
    while let Some(event) = window.next() { //anytime snake move, it cleans the window
        if let Some(Button::Keyboard(key)) = event.press_args() {
            game.key_pressed(key);
        }
        window.draw_2d(&event, |c, g, _|{
            clear(BACK_COLOR, g); //clear the window
            game.draw(&c, g);
        });

        event.update(|arg| {
            game.update(arg.dt); //dt is delta time in sec
        });
    }
}
