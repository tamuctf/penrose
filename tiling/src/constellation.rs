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

use std::collections::{BTreeMap, BTreeSet};
use std::f64::consts::TAU;
use std::iter::once;

use arrayvec::ArrayVec;
use euclid::default::{Point2D, Transform2D, Vector2D};
use euclid::Angle;
use itertools::Itertools;

use crate::fivefold::bar_num;

use super::constants::epsilon;
use super::fivefold::FiveFold;
use super::intersection_point::IntersectionPoint;

pub(crate) type PointGraph = BTreeMap<IntersectionPoint, BTreeSet<IntersectionPoint>>;

trait Consume {
    fn consume(&mut self, other: Self);
}

impl Consume for PointGraph {
    fn consume(&mut self, other: Self) {
        for (primary, mut secondaries) in other.into_iter() {
            match self.get_mut(&primary) {
                None => {
                    self.insert(primary.clone(), secondaries);
                }
                Some(existing) => {
                    existing.append(&mut secondaries);
                }
            }
        }
    }
}

fn pair_scan(points: &BTreeSet<IntersectionPoint>, delta: f64) -> PointGraph {
    let mut pairs = BTreeMap::new();

    for (primary, secondary) in points.iter().tuple_combinations() {
        if (Point2D::from(primary).distance_to(Point2D::from(secondary)) - delta).abs()
            < epsilon::<f64>()
        {
            // pre-sort
            let (primary, secondary) = if primary < secondary {
                (primary, secondary)
            } else {
                (secondary, primary)
            };
            match pairs.get_mut(primary) {
                None => {
                    let mut set = BTreeSet::new();
                    set.insert(secondary.clone());
                    pairs.insert(primary.clone(), set);
                }
                Some(set) => {
                    set.insert(secondary.clone());
                }
            }
        }
    }

    pairs
}

fn transform(real: &[IntersectionPoint; 2], test: [&IntersectionPoint; 2]) -> Transform2D<f64> {
    let real_theta = (real[1].y() - real[0].y())
        .atan2(real[1].x() - real[0].x())
        .rem_euclid(TAU);
    let test_theta = (test[1].y() - test[0].y())
        .atan2(test[1].x() - test[0].x())
        .rem_euclid(TAU);

    let theta = test_theta - real_theta;

    Transform2D::identity()
        .pre_translate(Vector2D::new(test[0].x(), test[0].y()))
        .pre_rotate(Angle::radians(theta))
        .pre_translate(Vector2D::new(-real[0].x(), -real[0].y()))
}

pub(crate) fn test_required(
    points: &BTreeSet<IntersectionPoint>,
    plane: &FiveFold,
    pair: [&IntersectionPoint; 2],
    key_pair: &[IntersectionPoint; 2],
    pattern: &[IntersectionPoint],
) -> Option<Transform2D<f64>> {
    let map = transform(key_pair, pair);

    for unmapped in pattern {
        let mapped = plane.intersection_point(map.transform_point(unmapped.into()));
        if let Some(mapped) = mapped {
            if !points.contains(&mapped) {
                return None;
            }
            let real_diff = unmapped.seq2().unwrap().rotation() - unmapped.seq1().unwrap().rotation();
            let test_diff = mapped.seq2().unwrap().rotation() - mapped.seq1().unwrap().rotation();

            if real_diff != test_diff && real_diff + test_diff != TAU {
                return None;
            }
        } else {
            return None;
        }
    }

    Some(map)
}

pub(crate) fn map_optional(
    point: &IntersectionPoint,
    map: &Transform2D<f64>,
    plane: &FiveFold,
    amount: usize,
) -> Option<IntersectionPoint> {
    let mapped = map.transform_point(point.into());

    let sequences = plane
        .sequences()
        .iter()
        .enumerate()
        .filter(|(_, ms)| plane.is_forced(mapped.into(), ms))
        .collect::<ArrayVec<_, 2>>();
    if sequences.is_full() {
        let bars = sequences
            .iter()
            .map(|(_, ms)| bar_num(mapped.into(), ms))
            .collect::<ArrayVec<_, 2>>();

        Some(IntersectionPoint::new_with_point(
            sequences[0].1,
            bars[0],
            sequences[1].1,
            bars[1],
            point.into(),
        ))
    } else if let Some((index, _)) = sequences.first() {
        let mut temp = IntersectionPoint::incomplete(mapped);
        temp.data.seq1 = Some(plane.sequences()[(index + amount) % 5].clone());
        Some(temp)
    } else {
        None
    }
}

pub trait Constellation {
    fn delta() -> f64;
    fn key_pair() -> &'static [IntersectionPoint; 2];
    fn pattern() -> &'static [IntersectionPoint];
    fn test_pair(
        points: &BTreeSet<IntersectionPoint>,
        plane: &FiveFold,
        pair: [&IntersectionPoint; 2],
    ) -> Option<Self>
    where
        Self: Sized;

    fn mapping(&self) -> Transform2D<f64>;

    fn force_bars(&self, plane: &mut FiveFold) -> bool;

    fn constellations(
        points: &BTreeSet<IntersectionPoint>,
        plane: &FiveFold,
        boundaries: Option<&[IntersectionPoint]>,
    ) -> Vec<Self>
    where
        Self: Sized,
    {
        let pairs = if let Some(boundaries) = boundaries.filter(|boundaries| boundaries.len() >= 2)
        {
            let mut pairs = PointGraph::new();

            for (old, current) in boundaries.iter().tuple_windows() {
                let slice: BTreeSet<IntersectionPoint> = points
                    .iter()
                    .skip_while(|&p| p < old)
                    .take_while(|&p| p <= current)
                    .cloned()
                    .collect();
                pairs.consume(pair_scan(&slice, Self::delta()));

                // println!("{}", pairs.len());
                // println!("{}", pairs.values().map(|item| item.len()).sum::<usize>());
            }

            let last = boundaries.last().unwrap();
            let slice: BTreeSet<IntersectionPoint> =
                points.iter().skip_while(|&p| p < last).cloned().collect();
            // println!("{} => {}", points.len(), slice.len());
            pairs.consume(pair_scan(&slice, Self::delta()));

            // println!("{}", pairs.len());
            // println!("{}", pairs.values().map(|item| item.len()).sum::<usize>());

            pairs
        } else {
            pair_scan(points, Self::delta())
        };

        let mut constellations = Vec::new();

        for (primary, secondary) in pairs
            .iter()
            .flat_map(|(primary, secondaries)| once(primary).cartesian_product(secondaries.iter()))
        {
            if let Some(found) = Self::test_pair(points, plane, [primary, secondary]) {
                constellations.push(found);
            } else if let Some(found) = Self::test_pair(points, plane, [secondary, primary]) {
                constellations.push(found);
            }
        }

        constellations
    }
}

#[cfg(test)]
mod test {
    use euclid::default::Point2D;

    use super::*;

    #[test]
    #[ignore]
    fn basic_affine() {
        let point = Point2D::new(3f64, 3f64);
        let other = Point2D::new(1f64, 0f64);

        let theta = (-45f64).to_radians();

        let map = Transform2D::new(
            theta.cos(),
            -theta.sin(),
            0f64,
            theta.sin(),
            theta.cos(),
            0f64,
        );

        println!("{:?}", map);
        println!("{:?}", map.transform_point(point));
        println!("{:?}", map.transform_point(other));
    }
}
