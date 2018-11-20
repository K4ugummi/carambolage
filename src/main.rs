// This file is part of Carambolage.

// Carambolage is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Carambolage is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Carambolage.  If not, see <http://www.gnu.org/licenses/>.
extern crate getopts;
extern crate gl;
extern crate glfw;
extern crate image;
extern crate log;
extern crate nalgebra;
extern crate ncollide3d;
extern crate rodio;
extern crate serde;
extern crate serde_derive;
extern crate simplelog;
extern crate time;
extern crate tobj;

use simplelog::*;

mod game;
mod grphx;
mod physx;
mod util;

use game::{Game, GameSettings};
use getopts::{Matches, Options};
use log::info;
use std::env;
use std::fs::File;

fn main() {
    // Read command line arguments.
    let args: Vec<String> = env::args().collect();

    // Set command line options.
    let opts = get_options();
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    // Filter settings
    let game_settings = match_options(&matches);

    let terminal_log_config = Config {
        time: Some(Level::Error),
        target: Some(Level::Debug),
        ..Default::default()
    };
    let write_log_config = Config {
        time: Some(Level::Error),
        target: Some(Level::Debug),
        ..Default::default()
    };
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Warn, terminal_log_config).unwrap(),
        WriteLogger::new(LevelFilter::Debug, write_log_config, File::create("carambolage.log").unwrap()),
    ]).unwrap();

    // Start the game
    info!("Starting game");
    let mut game = Game::new(game_settings);
    game.run();
}

fn get_options() -> Options {
    let mut opts = Options::new();
    opts.optflag("f", "fullscreen", "enable fullscreen mode");
    opts.optopt("w", "width", "set window width", "WIDTH");
    opts.optopt("h", "height", "set window height", "HEIGHT");
    opts.optopt("m", "map", "set the startup map by id", "MAP");
    opts.optopt("l", "limit-fps", "set max game fps [0 = unlimited]", "FPS");
    opts
}

fn match_options(matches: &Matches) -> GameSettings {
    let mut game_settings: GameSettings = Default::default();
    if matches.opt_present("f") {
        game_settings.is_fullscreen = true;
    }
    if matches.opt_str("w").is_some() {
        game_settings.width = matches.opt_str("w").unwrap().parse().unwrap();
    }
    if matches.opt_str("h").is_some() {
        game_settings.height = matches.opt_str("h").unwrap().parse().unwrap();
    }
    if matches.opt_str("m").is_some() {
        game_settings.map = matches.opt_str("m").unwrap().parse().unwrap();
    }
    if matches.opt_str("l").is_some() {
        game_settings.fps = matches.opt_str("l").unwrap().parse().unwrap();
    }
    game_settings
}

#[cfg(test)]
mod tests {
    use super::{get_options, match_options};

    #[test]
    fn arguments() {
        let args: Vec<String> = vec![
            String::from("./carambolage"),
            String::from("-f"),
            String::from("-w"),
            String::from("1920"),
            String::from("-h"),
            String::from("1080"),
            String::from("-l"),
            String::from("60"),
            String::from("-m"),
            String::from("1"),
        ];
        let opts = get_options();
        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => panic!(f.to_string()),
        };

        let settings = match_options(&matches);

        assert_eq!(settings.is_fullscreen, true);
        assert_eq!(settings.width, 1920);
        assert_eq!(settings.height, 1080);
        assert_eq!(settings.fps, 60);
        assert_eq!(settings.map, 1);
    }
}
