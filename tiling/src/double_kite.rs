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

use std::collections::BTreeSet;

use euclid::default::Transform2D;
use lazy_static::lazy_static;

use crate::musical_sequence::BarBound;

use super::constellation::{map_optional, test_required, Constellation};
use super::fivefold::{intersection_point, FiveFold};
use super::intersection_point::IntersectionPoint;
use std::borrow::Borrow;

lazy_static! {
    static ref PATTERN: [IntersectionPoint; 5] = {
        let plane = FiveFold::deuce_configuration();
        [
            intersection_point(&plane.sequences()[0], 0, &plane.sequences()[1], 0),
            intersection_point(&plane.sequences()[0], 0, &plane.sequences()[2], 0),
            intersection_point(&plane.sequences()[0], 0, &plane.sequences()[4], 0),
            intersection_point(&plane.sequences()[2], 0, &plane.sequences()[4], 0),
            intersection_point(&plane.sequences()[3], 0, &plane.sequences()[4], 0),
        ]
    };
    static ref KEY_PAIR: [IntersectionPoint; 2] = [PATTERN[1].clone(), PATTERN[3].clone(),];
    static ref DELTA: f64 = KEY_PAIR[0].point().distance_to(KEY_PAIR[1].point());
    static ref FORCE: IntersectionPoint = {
        let mut plane = FiveFold::deuce_configuration();
        plane.sequences_mut()[2].force(-1, BarBound::Shorter);

        intersection_point(&plane.sequences()[2], -1, &plane.sequences()[1], 0)
    };
}

#[derive(Debug, Copy, Clone)]
pub struct DoubleKite {
    mapping: Transform2D<f64>,
}

impl DoubleKite {
    fn new(mapping: Transform2D<f64>) -> Self {
        Self { mapping }
    }
}

impl Constellation for DoubleKite {
    fn delta() -> f64 {
        *DELTA
    }

    fn key_pair() -> &'static [IntersectionPoint; 2] {
        &*KEY_PAIR
    }

    fn pattern() -> &'static [IntersectionPoint] {
        &*PATTERN
    }

    fn test_pair(
        points: &BTreeSet<&IntersectionPoint>,
        plane: &FiveFold,
        pair: [&IntersectionPoint; 2],
    ) -> Option<Self>
    where
        Self: Sized,
    {
        let mapping = test_required(points, plane, pair, Self::key_pair(), Self::pattern());

        mapping.map(DoubleKite::new)
    }

    fn mapping(&self) -> Transform2D<f64> {
        self.mapping
    }

    fn force_bars(&self, plane: &mut FiveFold) -> bool {
        let new = map_optional(&*FORCE, &self.mapping, plane, 1).unwrap();

        if new.seq2().is_none() {
            return plane.force_point(new.borrow().point(), new.seq1().as_ref().unwrap());
        }
        false
    }
}
