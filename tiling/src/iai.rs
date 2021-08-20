use euclid::default::Point2D;

use euclid::default::Box2D;
use iai::black_box;
use penrose_tiling::{compute_area, FiveFold, MatchList};

fn render(bounds: &Box2D<f64>, mut plane: FiveFold) -> MatchList {
    compute_area(&mut plane, bounds)
}

fn ace() {
    let plane = FiveFold::ace_configuration();
    let bounds = Box2D::new(Point2D::new(-8., -4.), Point2D::new(8., 4.));

    render(&bounds, black_box(plane));
}

fn deuce() {
    let plane = FiveFold::deuce_configuration();
    let bounds = Box2D::new(Point2D::new(-40., -20.), Point2D::new(40., 20.));

    render(&bounds, black_box(plane));
}

fn sun() {
    let plane = FiveFold::sun_configuration();
    let bounds = Box2D::new(Point2D::new(-14., -7.), Point2D::new(14., 7.));

    render(&bounds, black_box(plane));
}

fn star() {
    let plane = FiveFold::star_configuration();
    let bounds = Box2D::new(Point2D::new(-40., -20.), Point2D::new(40., 20.));

    render(&bounds, black_box(plane));
}

fn jack() {
    let plane = FiveFold::jack_configuration();
    let bounds = Box2D::new(Point2D::new(-40., -20.), Point2D::new(40., 20.));

    render(&bounds, black_box(plane));
}

fn queen() {
    let plane = FiveFold::queen_configuration();
    let bounds = Box2D::new(Point2D::new(-40., -20.), Point2D::new(40., 20.));

    render(&bounds, black_box(plane));
}

fn king() {
    let plane = FiveFold::king_configuration();
    let bounds = Box2D::new(Point2D::new(-40., -20.), Point2D::new(40., 20.));

    render(&bounds, black_box(plane));
}

iai::main!(ace, deuce, sun, star, jack, queen, king);
