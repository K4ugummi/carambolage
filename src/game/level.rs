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
#![macro_use]
//use super::tile::{Tile, TileType};
use super::model::Model;

use nalgebra::Matrix4;

pub struct Level {
    model: Model,
    matrix: Matrix4<f32>,
}

impl Level {
    pub fn new(file: &str) -> Level {
        debug!("New from {}", file);
        let model = Model::new(file, "racetrack.png");
        let matrix = Matrix4::identity();

        Level { model, matrix }
    }

    pub fn draw(&self, view: &Matrix4<f32>, projection: &Matrix4<f32>) {
        self.model.draw(&self.matrix, view, projection);
    }
}
