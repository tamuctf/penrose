use std::collections::BTreeSet;
use std::f64::consts::PI;

use euclid::default::Box2D;
use euclid::default::Point2D;
use euclid::default::Transform2D;
use lazy_static::lazy_static;

use super::constants::*;
use super::constellation::{test_required, Constellation};
use super::fivefold::{intersection_point, FiveFold};
use super::intersection_point::IntersectionPoint;
use super::shape::{Shape, Triangle};

lazy_static! {
    static ref PATTERN: [IntersectionPoint; 3] = {
        let plane = FiveFold::sun_configuration();
        [
            intersection_point(&plane.sequences()[0], 0, &plane.sequences()[1], 0),
            intersection_point(&plane.sequences()[0], 0, &plane.sequences()[4], 0),
            intersection_point(&plane.sequences()[1], 0, &plane.sequences()[4], 0),
        ]
    };
    static ref KEY_PAIR: [IntersectionPoint; 2] = [PATTERN[0].clone(), PATTERN[1].clone()];
    static ref DELTA: f64 = Point2D::from(&KEY_PAIR[0]).distance_to((&KEY_PAIR[1]).into());
    static ref TRIANGLES: [Triangle; 2] = {
        let corner_x = (PI / 5f64).cos() * (minnick_x::<f64>() + minnick_y::<f64>());
        let corner_y = (PI / 5f64).sin() * (minnick_x::<f64>() + minnick_y::<f64>());

        let top_x = minnick_b::<f64>() + minnick_e::<f64>();

        [
            Triangle {
                a: Point2D::new(0f64, 0f64),
                b: Point2D::new(top_x, 0f64),
                c: Point2D::new(corner_x, corner_y),
            },
            Triangle {
                a: Point2D::new(0f64, 0f64),
                b: Point2D::new(top_x, 0f64),
                c: Point2D::new(corner_x, -corner_y),
            },
        ]
    };
    static ref BOUNDING_BOX: Box2D<f64> = Box2D::new(
        Point2D::new(0f64, TRIANGLES[1].c.y),
        Point2D::new(TRIANGLES[0].b.x, TRIANGLES[0].c.y),
    );
}

#[derive(Debug, Copy, Clone)]
pub struct Kite {
    mapping: Transform2D<f64>,
}

impl Kite {
    fn new(mapping: Transform2D<f64>) -> Self {
        Self { mapping }
    }
}

impl Constellation for Kite {
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
    ) -> Option<Self> {
        let mapping = test_required(points, plane, pair, Self::key_pair(), Self::pattern());

        mapping.map(|mapping| Kite::new(mapping))
    }

    fn mapping(&self) -> Transform2D<f64> {
        self.mapping
    }

    fn force_bars(&self, _: &mut FiveFold) -> bool {
        false
    }
}

impl Shape for Kite {
    fn contains(&self, point: Point2D<f64>) -> bool {
        let inverse = self.mapping.inverse().unwrap();
        let point = inverse.transform_point(point);

        // heuristic: check if we're even in the bounding box
        if BOUNDING_BOX.contains(point) {
            if point.y >= 0f64 {
                TRIANGLES[0].contains(point)
            } else {
                TRIANGLES[1].contains(point)
            }
        } else {
            false
        }
    }

    fn path(&self) -> Vec<Point2D<f64>> {
        vec![
            self.mapping.transform_point(TRIANGLES[0].a),
            self.mapping.transform_point(TRIANGLES[0].c),
            self.mapping.transform_point(TRIANGLES[0].b),
            self.mapping.transform_point(TRIANGLES[1].c),
        ]
    }
}
