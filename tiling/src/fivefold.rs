use std::borrow::{Borrow, BorrowMut};
use std::collections::BTreeSet;
use std::f64::consts::{PI, TAU};

use arrayvec::ArrayVec;
use euclid::default::{Box2D, Point2D};
use itertools::Itertools;

use crate::musical_sequence::BarBound;

use super::constants::*;
use super::intersection_point::IntersectionPoint;
use super::musical_sequence::BarNumber;
use super::musical_sequence::MusicalSequence;
use rustc_hash::FxHashMap;

const N: usize = 5;

#[derive(Debug, Clone)]
pub struct FiveFold {
    cache: FxHashMap<(u64, BarNumber, u64, BarNumber), IntersectionPoint>,
    sequences: ArrayVec<MusicalSequence, N>,
    last_forced: Option<f64>,
}

/// expect some bars :)
fn expected_intersections(bounds: &Box2D<f64>) -> usize {
    let avg_bar = (1. * short::<f64>() + golden_ratio::<f64>() * long::<f64>())
        / (1. + golden_ratio::<f64>());
    let range = f64::max(bounds.max.x - bounds.min.x, bounds.max.y - bounds.min.y);

    let bars = range / avg_bar;

    // some witchcraft:
    // - 10 for handshakes -- number of intersections that 5 non-parallel bars on their own have
    // - bars^2 because for each group of bars there are bars^2 intersections
    10 * (bars * bars) as usize
}

fn intersection(p1: Point2D<f64>, rot1: f64, p2: Point2D<f64>, rot2: f64) -> Option<Point2D<f64>> {
    if rot1 == rot2 {
        return None;
    }

    let slope1 = rot1.tan();
    let int1 = p1.y - (slope1 * p1.x);
    let slope2 = rot2.tan();
    let int2 = p2.y - (slope2 * p2.x);

    let x_int;
    let y_int;

    if rot1 == PI / 2f64 {
        x_int = p1.x;
        y_int = slope2 * x_int + int2;
    } else {
        if rot2 == PI / 2f64 {
            x_int = p2.x;
        } else {
            x_int = (int2 - int1) / (slope1 - slope2);
        }
        y_int = slope1 * x_int + int1;
    }

    Some(Point2D::new(x_int, y_int))
}

fn nearest_coords(x: f64, y: f64, ms: &MusicalSequence) -> Point2D<f64> {
    let bar_point = Point2D::new(x, y);
    let bar_r = ms.rotation() + PI / 2f64;

    let axis_point = Point2D::new(ms.center_x(), ms.center_y());
    let axis_r = ms.rotation();

    intersection(bar_point, bar_r, axis_point, axis_r)
        .expect("Presence guaranteed by discovering nearest")
}

fn nearest_point(p: Point2D<f64>, ms: &MusicalSequence) -> Point2D<f64> {
    nearest_coords(p.x, p.y, ms)
}

fn distance_along(p: Point2D<f64>, ms: &MusicalSequence) -> f64 {
    let center = Point2D::new(ms.center_x(), ms.center_y());
    let distance = center.distance_to(p);

    let theta = (p.y - center.y).atan2(p.x - center.x).rem_euclid(TAU);

    ((theta - ms.rotation()).abs() > epsilon::<f64>())
        .then(|| -distance)
        .unwrap_or(distance)
}

pub(crate) fn bar_num(p: Point2D<f64>, ms: &MusicalSequence) -> BarNumber {
    let seq_point = nearest_point(p, ms);
    let distance = distance_along(seq_point, ms);

    ms.get_bar(distance)
}

fn distance_to_point(ms: &MusicalSequence, distance: f64) -> Point2D<f64> {
    let angle = ms.rotation();
    let x = distance * angle.cos() + ms.center_x();
    let y = distance * angle.sin() + ms.center_y();

    Point2D::new(x, y)
}

fn bar_to_point(ms: &MusicalSequence, bar: BarNumber) -> Point2D<f64> {
    distance_to_point(ms, ms.get_bar_distance(bar))
}

fn bars(area: &Box2D<f64>, ms: &MusicalSequence, forced: bool) -> Vec<BarNumber> {
    let (first, last) = [area.min.x, area.max.x]
        .iter()
        .copied()
        .cartesian_product([area.min.y, area.max.y])
        .map(|(x, y)| nearest_coords(x, y, ms))
        .map(|p| distance_along(p, ms))
        .minmax()
        .into_option()
        .unwrap();

    let bars = ms.get_bar(first)..=ms.get_bar(last);

    if forced {
        ms.forced_bars(bars)
    } else {
        ms.unforced_bars(bars)
    }
}

fn forced_bars(area: &Box2D<f64>, ms: &MusicalSequence) -> Vec<BarNumber> {
    bars(area, ms, true)
}

pub(crate) fn intersection_point(
    a: &MusicalSequence,
    a_bar: BarNumber,
    b: &MusicalSequence,
    b_bar: BarNumber,
) -> IntersectionPoint {
    let a_point = bar_to_point(a, a_bar);
    let b_point = bar_to_point(b, b_bar);

    let intersection = intersection(
        a_point,
        a.rotation() + PI / 2f64,
        b_point,
        b.rotation() + PI / 2f64,
    )
    .expect("Guaranteed by previous calculation");

    IntersectionPoint::new_with_point(a, a_bar, b, b_bar, intersection)
}

impl FiveFold {
    pub fn new() -> Self {
        Self {
            cache: FxHashMap::default(),
            sequences: (0..N)
                .map(|i| MusicalSequence::new_with_coords(0f64, 0f64, (i as f64 * TAU) / N as f64))
                .collect(),
            last_forced: None,
        }
    }

    pub fn ace_configuration() -> FiveFold {
        let mut plane = FiveFold::new();
        let seqs = plane.sequences_mut();

        seqs[0].set_zeroeth(minnick_a::<f64>());
        seqs[1].set_zeroeth(minnick_a::<f64>());
        seqs[2].set_zeroeth(-(minnick_x::<f64>() + minnick_y::<f64>() + minnick_z::<f64>()));
        seqs[3].set_zeroeth(-(minnick_x::<f64>() + minnick_y::<f64>() + minnick_z::<f64>()));
        seqs[4].set_zeroeth(minnick_a::<f64>());

        plane
    }

    pub fn deuce_configuration() -> FiveFold {
        let mut plane = FiveFold::new();
        let seqs = plane.sequences_mut();

        seqs[0].set_zeroeth(minnick_a::<f64>());
        seqs[1].set_zeroeth(-(minnick_x::<f64>() + minnick_y::<f64>() + minnick_z::<f64>()));
        seqs[2].set_zeroeth(-(minnick_x::<f64>() + minnick_y::<f64>() + minnick_z::<f64>()));
        seqs[3].set_zeroeth(-(minnick_x::<f64>() + minnick_y::<f64>() + minnick_z::<f64>()));
        seqs[4].set_zeroeth(minnick_a::<f64>());

        plane
    }

    pub fn sun_configuration() -> FiveFold {
        let mut plane = FiveFold::new();

        for ms in plane.sequences_mut() {
            ms.set_zeroeth(minnick_b::<f64>());
        }

        plane
    }

    pub fn star_configuration() -> FiveFold {
        let mut plane = FiveFold::new();

        for ms in plane.sequences_mut() {
            ms.set_zeroeth(-(minnick_x::<f64>() + minnick_y::<f64>() + minnick_z::<f64>()));
            ms.force(1, BarBound::Longer);
        }

        plane
    }

    pub fn jack_configuration() -> FiveFold {
        let mut plane = FiveFold::new();
        let seqs = plane.sequences_mut();

        seqs[0].set_zeroeth(minnick_e::<f64>());
        seqs[0].force(-1, BarBound::Longer);
        seqs[1].set_zeroeth(minnick_z::<f64>());
        seqs[1].force(-1, BarBound::Longer);
        seqs[2].set_zeroeth(minnick_b::<f64>());
        seqs[3].set_zeroeth(minnick_b::<f64>());
        seqs[4].set_zeroeth(minnick_z::<f64>());
        seqs[4].force(-1, BarBound::Longer);

        plane
    }

    pub fn queen_configuration() -> FiveFold {
        let mut plane = FiveFold::new();
        let seqs = plane.sequences_mut();

        seqs[0].set_zeroeth(minnick_a::<f64>());
        seqs[1].set_zeroeth(-minnick_w::<f64>());
        seqs[1].force(1, BarBound::Shorter);
        seqs[2].set_zeroeth(-(minnick_x::<f64>() + minnick_y::<f64>() + minnick_z::<f64>()));
        seqs[2].force(1, BarBound::Longer);
        seqs[3].set_zeroeth(-(minnick_x::<f64>() + minnick_y::<f64>() + minnick_z::<f64>()));
        seqs[3].force(1, BarBound::Longer);
        seqs[4].set_zeroeth(-minnick_w::<f64>());
        seqs[4].force(1, BarBound::Shorter);

        plane
    }

    pub fn king_configuration() -> FiveFold {
        let mut plane = FiveFold::new();
        let seqs = plane.sequences_mut();

        seqs[0].set_zeroeth(-minnick_w::<f64>());
        seqs[0].force(1, BarBound::Shorter);
        seqs[1].set_zeroeth(-(minnick_x::<f64>() + minnick_y::<f64>() + minnick_z::<f64>()));
        seqs[1].force(1, BarBound::Longer);
        seqs[2].set_zeroeth(-(minnick_x::<f64>() + minnick_y::<f64>() + minnick_z::<f64>()));
        seqs[2].force(1, BarBound::Longer);
        seqs[3].set_zeroeth(-(minnick_x::<f64>() + minnick_y::<f64>() + minnick_z::<f64>()));
        seqs[3].force(1, BarBound::Longer);
        seqs[4].set_zeroeth(-(minnick_x::<f64>() + minnick_y::<f64>() + minnick_z::<f64>()));
        seqs[4].force(1, BarBound::Longer);

        plane
    }

    pub(crate) fn is_forced_at_coords(&self, x: f64, y: f64, ms: &MusicalSequence) -> bool {
        let forced = nearest_coords(x, y, ms);
        let distance = distance_along(forced, ms);
        let bar = ms.get_bar(distance);
        let bar_dist = ms.get_bar_distance(bar);

        ((distance - bar_dist).abs() <= epsilon::<f64>())
            .then(|| ms.is_forced(bar))
            .unwrap_or(false)
    }

    pub(crate) fn is_forced(&self, p: Point2D<f64>, ms: &MusicalSequence) -> bool {
        self.is_forced_at_coords(p.x, p.y, ms)
    }

    pub(crate) fn intersection_point(&self, p: Point2D<f64>) -> Option<IntersectionPoint> {
        let sequences = self
            .sequences
            .iter()
            .filter(|&ms| self.is_forced(p, ms))
            .collect::<ArrayVec<_, 2>>();
        sequences.is_full().then(|| {
            let bars = sequences
                .iter()
                .map(|&ms| bar_num(p, ms))
                .collect::<ArrayVec<_, 2>>();

            IntersectionPoint::new_with_point(sequences[0], bars[0], sequences[1], bars[1], p)
        })
    }

    pub(crate) fn update_intersection_points(&mut self, bounds: &Box2D<f64>) {
        let expected = expected_intersections(bounds);
        if self.cache.capacity() < expected {
            self.cache.reserve(expected - self.cache.capacity())
        }

        if let Some(forced) = self.last_forced.take() {
            for (a, a_bar, b, b_bar) in self
                .sequences
                .iter()
                .map(|ms| (ms, forced_bars(&bounds, ms)))
                .tuple_combinations()
                .filter(|((a, _), (b, _))| a.rotation() == forced || b.rotation() == forced)
                .flat_map(|((a, a_bars), (b, b_bars))| {
                    a_bars
                        .into_iter()
                        .cartesian_product(b_bars)
                        .map(move |(a_bar, b_bar)| (a, a_bar, b, b_bar))
                })
            {
                let key = (a.rotation().to_bits(), a_bar, b.rotation().to_bits(), b_bar);
                self.cache
                    .entry(key)
                    .or_insert_with(|| intersection_point(a, a_bar, b, b_bar));
            }
        } else {
            for (a, a_bar, b, b_bar) in self
                .sequences
                .iter()
                .map(|ms| (ms, forced_bars(&bounds, ms)))
                .tuple_combinations()
                .flat_map(|((a, a_bars), (b, b_bars))| {
                    a_bars
                        .into_iter()
                        .cartesian_product(b_bars)
                        .map(move |(a_bar, b_bar)| (a, a_bar, b, b_bar))
                })
            {
                let key = (a.rotation().to_bits(), a_bar, b.rotation().to_bits(), b_bar);
                self.cache
                    .entry(key)
                    .or_insert_with(|| intersection_point(a, a_bar, b, b_bar));
            }
        }
    }

    pub(crate) fn intersection_points(
        &self,
        bounds: &Box2D<f64>,
    ) -> BTreeSet<&'_ IntersectionPoint> {
        let mut intermediate = BTreeSet::new();

        self.cache
            .values()
            .filter(|point| bounds.contains(point.point()))
            .all(|point| point.add_to(&mut intermediate));

        intermediate
    }

    pub(crate) fn sequences(&self) -> &[MusicalSequence] {
        self.sequences.borrow()
    }

    pub(crate) fn sequences_mut(&mut self) -> &mut [MusicalSequence] {
        self.sequences.borrow_mut()
    }

    pub(crate) fn force_point(&mut self, p: Point2D<f64>, ms: &MusicalSequence) -> bool {
        assert!(self.last_forced.is_none(), "Naughty!");
        let along = nearest_point(p, ms);
        let distance = distance_along(along, ms);

        let ms = self
            .sequences
            .iter_mut()
            .find(|sequence| ms.rotation() == sequence.rotation())
            .unwrap();
        if ms.force_at_distance(distance) {
            self.last_forced.replace(ms.rotation());
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use crate::musical_sequence::BarBound;

    use super::*;

    #[test]
    fn basic() {
        let mut fold = FiveFold::new();

        fold.sequences
            .iter_mut()
            .for_each(|ms| ms.force(1, BarBound::Shorter));

        fold.sequences
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| (a, b, intersection_point(a, 1, b, 1)))
            .for_each(|(a, b, point)| {
                println!(
                    "Intersection point between {}, {} at ({}, {})",
                    a.rotation(),
                    b.rotation(),
                    point.x(),
                    point.y(),
                )
            });

        let expected_intersections = vec![Point2D::new(0f64, 0f64); 10];
        let actual_intersections = fold.intersection_points(&Box2D::new(
            Point2D::new(-1.2, -1.2),
            Point2D::new(1.2, 1.2),
        ));

        assert_eq!(expected_intersections.len(), actual_intersections.len());
        expected_intersections
            .into_iter()
            .zip(actual_intersections.iter())
            .for_each(|(expected, actual)| assert_eq!(expected, actual.point()));
    }
}
