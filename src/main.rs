use std::error::Error;

use euclid::default::Box2D;
use euclid::default::Point2D;
use euclid::default::Transform2D;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;
use tiling::{compute_area, FiveFold, MatchList, Shape};

fn draw<T: Shape>(shape: T, colour: &str, transform: &Transform2D<f64>) -> Path {
    let mut iter = shape.path().into_iter();
    let mut data = Data::new().move_to(transform.transform_point(iter.next().unwrap()).to_tuple());
    for point in iter {
        data = data.line_to(transform.transform_point(point).to_tuple());
    }
    data = data.close();
    Path::new()
        .set("fill", colour)
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("stroke-linecap", "round")
        .set("d", data)
}

fn render(matches: MatchList, bounds: &Box2D<f64>, scale: f64) -> Document {
    let transform = Transform2D::translation(-bounds.min.x, -bounds.min.y).then_scale(scale, scale);
    let bounds = bounds
        .translate((-bounds.min.x, -bounds.min.y).into())
        .scale(scale, scale);
    matches
        .darts
        .into_iter()
        .map(|dart| draw(dart, "red", &transform))
        .chain(
            matches
                .kites
                .into_iter()
                .map(|kite| draw(kite, "green", &transform)),
        )
        .fold(
            Document::new().set(
                "viewbox",
                (bounds.min.x, bounds.min.y, bounds.max.x, bounds.max.y),
            ),
            |doc, path| doc.add(path),
        )
}

fn main() -> Result<(), Box<dyn Error>> {
    let configuration = std::env::args().skip(1).next().unwrap_or("king".to_owned());
    let mut plane = match configuration.as_str() {
        "ace" => FiveFold::ace_configuration(),
        "deuce" => FiveFold::deuce_configuration(),
        "sun" => FiveFold::sun_configuration(),
        "star" => FiveFold::star_configuration(),
        "jack" => FiveFold::jack_configuration(),
        "queen" => FiveFold::queen_configuration(),
        "king" => FiveFold::king_configuration(),
        _ => panic!("Invalid configuration selection"),
    };
    let bounds = Box2D::new(Point2D::new(-40., -20.), Point2D::new(40., 20.));
    let matches = compute_area(&mut plane, &bounds);

    svg::save(
        format!("{}.svg", configuration),
        &render(matches, &bounds, 20.),
    )
    .unwrap();

    Ok(())
}
