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

#[cfg(test)]
mod test {
    use std::io::Write;

    use euclid::default::Point2D;

    use crate::shape::Shape;

    use super::*;

    fn show_matches(bounds: &Box2D<f64>, mut plane: FiveFold) {
        let matches = compute_area(&mut plane, &bounds);

        let y_range = bounds.max.y - bounds.min.y;
        let x_range = bounds.max.x - bounds.min.x;

        let y_step = y_range / 50f64;
        let x_step = x_range / 200f64;
        let y_center = (bounds.max.y + bounds.min.y + y_step) / 2f64;
        let x_center = (bounds.max.x + bounds.min.x + x_step) / 2f64;

        let mut res = String::from("\n");
        for y in -25..25 {
            for x in -100..100 {
                let point =
                    Point2D::new(x as f64 * x_step + x_center, y as f64 * y_step + y_center);

                if matches.kites.iter().any(|kite| kite.contains(point)) {
                    res += "X";
                } else if matches.darts.iter().any(|dart| dart.contains(point)) {
                    res += ":"
                } else {
                    res += " ";
                }
            }
            res += "\n";
        }
        res += "\n";
        let stdout = std::io::stdout();
        let mut lock = stdout.lock();
        lock.write_all(res.as_bytes()).unwrap();
        lock.flush().unwrap();
    }

    #[test]
    #[ignore]
    fn ace() {
        let plane = FiveFold::ace_configuration();
        let bounds = Box2D::new(Point2D::new(-8., -4.), Point2D::new(8., 4.));

        show_matches(&bounds, plane);
    }

    #[test]
    #[ignore]
    fn deuce() {
        let plane = FiveFold::deuce_configuration();
        let bounds = Box2D::new(Point2D::new(-40., -20.), Point2D::new(40., 20.));

        show_matches(&bounds, plane);
    }

    #[test]
    #[ignore]
    fn sun() {
        let plane = FiveFold::sun_configuration();
        let bounds = Box2D::new(Point2D::new(-14., -7.), Point2D::new(14., 7.));

        show_matches(&bounds, plane);
    }

    #[test]
    #[ignore]
    fn star() {
        let plane = FiveFold::star_configuration();
        let bounds = Box2D::new(Point2D::new(-40., -20.), Point2D::new(40., 20.));

        show_matches(&bounds, plane);
    }

    #[test]
    #[ignore]
    fn jack() {
        let plane = FiveFold::jack_configuration();
        let bounds = Box2D::new(Point2D::new(-40., -20.), Point2D::new(40., 20.));

        show_matches(&bounds, plane);
    }

    #[test]
    #[ignore]
    fn queen() {
        let plane = FiveFold::queen_configuration();
        let bounds = Box2D::new(Point2D::new(-40., -20.), Point2D::new(40., 20.));

        show_matches(&bounds, plane);
    }

    #[test]
    #[ignore]
    fn king() {
        let plane = FiveFold::king_configuration();
        let bounds = Box2D::new(Point2D::new(-40., -20.), Point2D::new(40., 20.));

        show_matches(&bounds, plane);
    }
}
