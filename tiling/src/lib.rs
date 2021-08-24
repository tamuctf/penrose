/*
 * Penrose: Penrose tiling generation, adjacency, and other miscellaneous APIs.
 * Copyright (C) 2021  TAMUctf
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use euclid::default::Box2D;

pub use constellation::Constellation;
pub use dart::Dart;
pub use double_kite::DoubleKite;
pub use fivefold::FiveFold;
pub use kite::Kite;
pub use musical_sequence::MusicalSequence;
pub use shape::Shape;

pub mod constants;
mod constellation;
mod dart;
mod double_kite;
mod fivefold;
mod intersection_point;
mod kite;
mod musical_sequence;
mod shape;

#[derive(Debug)]
pub struct MatchList {
    pub kites: Vec<Kite>,
    pub darts: Vec<Dart>,
}

fn force_new<T: Constellation + Sized>(plane: &mut FiveFold, constellations: &Vec<T>) -> bool {
    constellations
        .iter()
        .any(|constellation| constellation.force_bars(plane))
}

pub fn compute_area(plane: &mut FiveFold, bounds: &Box2D<f64>) -> MatchList {
    let mut darts = Vec::new();
    let mut double_kites = Vec::new();
    let mut boundaries = Vec::new();

    let (points, boundaries) = loop {
        darts.clear();
        double_kites.clear();
        boundaries.clear();

        let points = plane.intersection_points(bounds);

        let mut layer = -1f64;
        let mut theta = -1f64;

        for point in points.iter() {
            if point.box_layer != layer || point.box_theta != theta {
                layer = point.box_layer;
                theta = point.box_theta;

                boundaries.push(point.clone());
            }
        }

        Dart::constellations(&points, plane, Some(&boundaries), &mut darts);
        DoubleKite::constellations(&points, plane, Some(&boundaries), &mut double_kites);

        if !(force_new(plane, &darts) || force_new(plane, &double_kites)) {
            break (points, boundaries);
        }
    };

    let mut kites = Vec::new();
    Kite::constellations(&points, plane, Some(&boundaries), &mut kites);

    MatchList { kites, darts }
}
