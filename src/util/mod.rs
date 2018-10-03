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
// along with Foobar.  If not, see <http://www.gnu.org/licenses/>.
use nalgebra::{clamp, Vector2, Vector3};

/// Interpolate from a to b with a given factor.
/// factor = 0.0 returns a
/// factor = 1.0 returns b
pub trait Lerp {
    fn lerp(a: &Self, b: &Self, factor: f32) -> Self;
}

impl Lerp for Vector2<f32> {
    fn lerp(a: &Self, b: &Self, factor: f32) -> Self {
        let f = clamp(factor, 0., 1.);
        a + (b - a) * f
    }
}

impl Lerp for Vector3<f32> {
    fn lerp(a: &Self, b: &Self, factor: f32) -> Self {
        let f = clamp(factor, 0., 1.);
        a + (b - a) * f
    }
}

impl Lerp for f32 {
    fn lerp(a: &Self, b: &Self, factor: f32) -> Self {
        let f = clamp(factor, 0., 1.);
        a + (b - a) * f
    }
}
