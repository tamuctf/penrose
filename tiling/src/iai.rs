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

use euclid::default::Point2D;

use euclid::default::Box2D;
use penrose_tiling::{FiveFold, Tiling};

fn render(mut tiling: Tiling) {
    tiling.compute_area();
}

fn ace() {
    let plane = FiveFold::ace_configuration();
    let bounds = Box2D::new(Point2D::new(-8., -4.), Point2D::new(8., 4.));

    render(Tiling::new(plane, bounds))
}

fn deuce() {
    let plane = FiveFold::deuce_configuration();
    let bounds = Box2D::new(Point2D::new(-40., -20.), Point2D::new(40., 20.));

    render(Tiling::new(plane, bounds))
}

fn sun() {
    let plane = FiveFold::sun_configuration();
    let bounds = Box2D::new(Point2D::new(-14., -7.), Point2D::new(14., 7.));

    render(Tiling::new(plane, bounds))
}

fn star() {
    let plane = FiveFold::star_configuration();
    let bounds = Box2D::new(Point2D::new(-40., -20.), Point2D::new(40., 20.));

    render(Tiling::new(plane, bounds))
}

fn jack() {
    let plane = FiveFold::jack_configuration();
    let bounds = Box2D::new(Point2D::new(-40., -20.), Point2D::new(40., 20.));

    render(Tiling::new(plane, bounds))
}

fn queen() {
    let plane = FiveFold::queen_configuration();
    let bounds = Box2D::new(Point2D::new(-40., -20.), Point2D::new(40., 20.));

    render(Tiling::new(plane, bounds))
}

fn king() {
    let plane = FiveFold::king_configuration();
    let bounds = Box2D::new(Point2D::new(-40., -20.), Point2D::new(40., 20.));

    render(Tiling::new(plane, bounds))
}

iai::main!(ace, deuce, sun, star, jack, queen, king);
