use std::collections::BTreeSet;

use euclid::default::Point2D;
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
    static ref DELTA: f64 = Point2D::from(&KEY_PAIR[0]).distance_to((&KEY_PAIR[1]).into());
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
        points: &BTreeSet<IntersectionPoint>,
        plane: &FiveFold,
        pair: [&IntersectionPoint; 2],
    ) -> Option<Self>
    where
        Self: Sized,
    {
        let mapping = test_required(points, plane, pair, Self::key_pair(), Self::pattern());

        mapping.map(|mapping| DoubleKite::new(mapping))
    }

    fn mapping(&self) -> Transform2D<f64> {
        self.mapping
    }

    fn force_bars(&self, plane: &mut FiveFold) -> bool {
        let new = map_optional(&*FORCE, &self.mapping, plane, 1).unwrap();

        if new.seq2.is_none() {
            return plane.force_point(new.borrow().into(), new.seq1.as_ref().unwrap());
        }
        false
    }
}
