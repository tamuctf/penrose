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

use euclid::default::Box2D;

use crate::intersection_point::IntersectionPoint;
use constants::*;
pub use constellation::Constellation;
pub use dart::Dart;
pub use double_kite::DoubleKite;
pub use fivefold::FiveFold;
pub use kite::Kite;
pub use musical_sequence::MusicalSequence;
pub use shape::Shape;
pub use tiling::MatchList;
pub use tiling::Tiling;

pub mod constants;
mod constellation;
mod dart;
mod double_kite;
mod fivefold;
mod intersection_point;
mod kite;
mod musical_sequence;
mod shape;
mod tiling;
