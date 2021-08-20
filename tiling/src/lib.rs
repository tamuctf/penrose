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
    let kites;
    let mut darts;
    let mut double_kites;

    let (points, boundaries) = loop {
        let points = plane.intersection_points(bounds);

        let mut boundaries = Vec::new();
        let mut layer = -1f64;
        let mut theta = -1f64;

        for point in points.iter() {
            if point.box_layer != layer || point.box_theta != theta {
                layer = point.box_layer;
                theta = point.box_theta;

                boundaries.push(point.clone());
            }
        }

        darts = Dart::constellations(&points, plane, Some(&boundaries));
        double_kites = DoubleKite::constellations(&points, plane, Some(&boundaries));

        if !(force_new(plane, &darts) || force_new(plane, &double_kites)) {
            break (points, boundaries);
        }
    };

    kites = Kite::constellations(&points, plane, Some(&boundaries));

    MatchList { kites, darts }
}
