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
extern crate nalgebra;
extern crate rand;
extern crate time;

mod game;
mod util;

use game::{Game, GameSettings};
use getopts::Options;
use std::env;

fn main() {
    // Read command line arguments.
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("f", "fullscreen", "enable fullscreen mode");
    opts.optopt("w", "width", "set window width", "WIDTH");
    opts.optopt("h", "height", "set window height", "HEIGHT");
    opts.optopt("l", "limit-fps", "set max game fps [0 = unlimited]", "FPS");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

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
    if matches.opt_str("l").is_some() {
        game_settings.fps = matches.opt_str("l").unwrap().parse().unwrap();
    }

    let mut game = Game::new(game_settings);
    game.run();
}

#[cfg(test)]
mod tests {
    use getopts::Options;

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
        ];
        let mut opts = Options::new();
        opts.optflag("f", "fullscreen", "enable fullscreen mode");
        opts.optopt("w", "width", "set window width", "WIDTH");
        opts.optopt("h", "height", "set window height", "HEIGHT");
        opts.optopt("l", "limit-fps", "set max game fps [0 = unlimited]", "FPS");
        let matches = match opts.parse(&args[1..]) {
            Ok(m) => m,
            Err(f) => panic!(f.to_string()),
        };
        let is_fullscreen = matches.opt_present("f");
        let width: u32 = matches.opt_str("w").unwrap().parse().unwrap();
        let height: u32 = matches.opt_str("h").unwrap().parse().unwrap();
        let fps_limit: u32 = matches.opt_str("l").unwrap().parse().unwrap();

        assert_eq!(is_fullscreen, true);
        assert_eq!(width, 1920);
        assert_eq!(height, 1080);
        assert_eq!(fps_limit, 60);
    }
}
