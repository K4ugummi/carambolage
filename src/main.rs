extern crate nalgebra;
extern crate rand;
extern crate time;

mod game;
use game::Game;

fn main() {
    let mut game = Game::new();
    game.run();
}
