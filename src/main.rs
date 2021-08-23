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

use std::error::Error;

use euclid::default::Box2D;
use euclid::default::Point2D;
use euclid::default::Transform2D;
use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;
use tiling::{FiveFold, MatchList, Shape, Tiling};

fn draw<T: Shape<N>, const N: usize>(shape: T, colour: &str, transform: &Transform2D<f64>) -> Path {
    // array iteration by value doesn't resolve properly until Edition 2021 rolls around
    let mut iter = <[_; N]>::into_iter(shape.path());
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
    let mut tiling = Tiling::new(plane, bounds);
    let matches = tiling.compute_area();

    svg::save(
        format!("{}.svg", configuration),
        &render(matches, &bounds, 20.),
    )
    .unwrap();

    Ok(())
}
