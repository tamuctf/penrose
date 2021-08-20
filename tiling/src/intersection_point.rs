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

use std::cmp::{max_by, Ordering};
use std::collections::BTreeSet;

use euclid::default::Point2D;

use super::constants::*;
use super::musical_sequence::BarNumber;
use super::musical_sequence::MusicalSequence;
use std::f64::consts::TAU;
use std::fmt::{Display, Formatter};

#[derive(Debug, Default, Clone)]
pub struct IntersectionPoint {
    pub x: f64,
    pub y: f64,
    pub(crate) seq1: Option<MusicalSequence>,
    pub(crate) bar1: BarNumber,
    pub(crate) seq2: Option<MusicalSequence>,
    pub(crate) bar2: BarNumber,
    pub(crate) box_layer: f64,
    pub(crate) box_theta: f64,
    dup_right: Option<Box<IntersectionPoint>>,
    dup_bottom: Option<Box<IntersectionPoint>>,
    dup_diagonal: Option<Box<IntersectionPoint>>,
}

fn box_info(x_boxes: f64, y_boxes: f64) -> (f64, f64) {
    (
        max_by(x_boxes.abs(), y_boxes.abs(), |&x1, x2| {
            x1.partial_cmp(x2).unwrap_or(Ordering::Equal)
        }),
        y_boxes.atan2(x_boxes).rem_euclid(TAU),
    )
}

impl IntersectionPoint {
    pub(crate) fn incomplete(point: Point2D<f64>) -> Self {
        Self {
            x: point.x,
            y: point.y,
            box_layer: -1f64,
            box_theta: -1f64,
            ..Default::default()
        }
    }

    pub(crate) fn new_with_point(
        seq1: &MusicalSequence,
        bar1: BarNumber,
        seq2: &MusicalSequence,
        bar2: BarNumber,
        point: Point2D<f64>,
    ) -> Self {
        Self::new(seq1, bar1, seq2, bar2, point.x, point.y)
    }

    fn new_derived(base: &Self, layer: f64, theta: f64) -> Self {
        Self {
            x: base.x,
            y: base.y,
            seq1: base.seq1,
            bar1: base.bar1,
            seq2: base.seq2,
            bar2: base.bar2,
            box_layer: layer,
            box_theta: theta,
            ..Default::default()
        }
    }

    fn new(
        seq1: &MusicalSequence,
        bar1: BarNumber,
        seq2: &MusicalSequence,
        bar2: BarNumber,
        x: f64,
        y: f64,
    ) -> Self {
        let (seq1, bar1, seq2, bar2) = if seq1.rotation() < seq2.rotation() {
            (Some(*seq1), bar1, Some(*seq2), bar2)
        } else {
            (Some(*seq2), bar2, Some(*seq1), bar1)
        };
        let mut res = Self {
            x,
            y,
            seq1,
            bar1,
            seq2,
            bar2,
            ..Default::default()
        };

        let x_boxes = (res.x - box_origin::<f64>()) / box_dim::<f64>();
        let y_boxes = (res.y - box_origin::<f64>()) / box_dim::<f64>();

        let x_floor = x_boxes.floor();
        let y_floor = y_boxes.floor();

        let x_rem = x_boxes - x_floor;
        let y_rem = y_boxes - y_floor;

        let x_overflow = (x_rem * box_dim::<f64>()) + box_overlap::<f64>();
        let y_overflow = (y_rem * box_dim::<f64>()) + box_overlap::<f64>();

        if x_overflow > box_dim::<f64>() && y_overflow > box_dim::<f64>() {
            let (layer, theta) = box_info(x_floor + 1f64, y_floor + 1f64);
            res.dup_diagonal
                .replace(Box::new(Self::new_derived(&res, layer, theta)));
        }
        if x_overflow > box_dim::<f64>() {
            let (layer, theta) = box_info(x_floor + 1f64, y_floor);
            res.dup_right
                .replace(Box::new(Self::new_derived(&res, layer, theta)));
        }
        if y_overflow > box_dim::<f64>() {
            let (layer, theta) = box_info(x_floor, y_floor + 1f64);
            res.dup_bottom
                .replace(Box::new(Self::new_derived(&res, layer, theta)));
        }

        let (layer, theta) = box_info(x_floor, y_floor);
        res.box_layer = layer;
        res.box_theta = theta;

        res
    }

    pub(crate) fn add_to(&self, store: &mut BTreeSet<Self>) -> bool {
        std::iter::once(self)
            .chain(self.dup_right.as_deref())
            .chain(self.dup_bottom.as_deref())
            .chain(self.dup_diagonal.as_deref())
            .all(|point| store.insert(point.clone()))
    }
}

impl From<&IntersectionPoint> for Point2D<f64> {
    fn from(point: &IntersectionPoint) -> Self {
        Point2D::new(point.x, point.y)
    }
}

impl Eq for IntersectionPoint {}

impl PartialEq<Self> for IntersectionPoint {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other).unwrap().is_eq()
    }
}

impl PartialOrd<Self> for IntersectionPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(
            self.box_layer
                .partial_cmp(&other.box_layer)
                .unwrap()
                .then(self.box_theta.partial_cmp(&other.box_theta).unwrap())
                .then(
                    self.seq1
                        .unwrap()
                        .rotation()
                        .partial_cmp(&other.seq1.unwrap().rotation())
                        .unwrap(),
                )
                .then(
                    self.seq2
                        .unwrap()
                        .rotation()
                        .partial_cmp(&other.seq2.unwrap().rotation())
                        .unwrap(),
                )
                .then(self.bar1.cmp(&other.bar1))
                .then(self.bar2.cmp(&other.bar2)),
        )
    }
}

impl Ord for IntersectionPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Display for IntersectionPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!("({}, {})", self.x, self.y))
    }
}

#[cfg(test)]
mod test {
    use crate::musical_sequence::BarBound;

    use super::*;

    #[test]
    #[ignore]
    fn test_point_generation() {
        let mut points = BTreeSet::new();

        let mut primary = MusicalSequence::new_with_coords(0.0, 0.0, 0.0);
        let mut secondary = MusicalSequence::new_with_coords(0.0, 0.0, 72f64.to_radians());

        primary.force(10, BarBound::Longer);
        secondary.force(10, BarBound::Shorter);

        for i in (0..100).step_by(10) {
            for j in (0..100).step_by(10) {
                let point = IntersectionPoint::new(&primary, i, &secondary, j, i as f64, j as f64);

                (!point.add_to(&mut points)).then(|| panic!("couldn't add all points"));
            }
        }

        for point in points {
            println!(
                "point box layer: {}  theta: {}",
                point.box_layer,
                point.box_theta.to_degrees()
            );
        }
    }
}
