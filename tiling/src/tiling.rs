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

use super::constants::*;
use super::constellation::Constellation;
use super::dart::Dart;
use super::double_kite::DoubleKite;
use super::fivefold::FiveFold;
use super::intersection_point::IntersectionPoint;
use super::kite::Kite;
use euclid::default::Box2D;
use rustc_hash::FxHashMap;

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

pub struct Tiling {
    plane: FiveFold,
    bounds: Box2D<f64>,
}

impl Tiling {
    pub fn new(plane: FiveFold, bounds: Box2D<f64>) -> Self {
        Self { plane, bounds }
    }

    pub fn compute_area(&mut self) -> MatchList {
        let mut kites;
        let mut darts;
        let mut double_kites;

        loop {
            self.plane.update_intersection_points(&self.bounds);
            {
                let points = self.plane.intersection_points(&self.bounds);

                let mut boundaries = Vec::new();
                let mut layer = -1f64;
                let mut theta = -1f64;

                for point in points.iter() {
                    if point.box_layer() != layer || point.box_theta() != theta {
                        layer = point.box_layer();
                        theta = point.box_theta();

                        boundaries.push(point.clone());
                    }
                }

                darts = Dart::constellations(&points, &self.plane, Some(&boundaries));
                double_kites = DoubleKite::constellations(&points, &self.plane, Some(&boundaries));
                kites = Kite::constellations(&points, &self.plane, Some(&boundaries));
            }

            if !(force_new(&mut self.plane, &darts) || force_new(&mut self.plane, &double_kites)) {
                break;
            }
        }

        MatchList { kites, darts }
    }
}
