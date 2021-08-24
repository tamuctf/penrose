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

use std::cmp::Ordering::Equal;
use std::ops::Range;

use itertools::*;

use super::constants::*;
use num_traits::ToPrimitive;

pub type BarNumber = i64;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum BarBound {
    Longer,
    Shorter,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct MusicalSequence {
    upper_x: BarNumber,
    upper_y: BarNumber,
    lower_x: BarNumber,
    lower_y: BarNumber,
    center_x: f64,
    center_y: f64,
    rotation: f64,
}

fn truncate_open(value: f64) -> BarNumber {
    value.floor() as BarNumber
}

fn truncate_closed(value: f64) -> BarNumber {
    let temp = value.floor() as BarNumber;
    if temp as f64 == value {
        temp - 1
    } else {
        temp
    }
}

fn ddist(shorts: BarNumber, longs: BarNumber, distance: f64) -> f64 {
    (distance - (shorts as f64 + (golden_ratio::<f64>() * longs as f64))).abs()
}

impl MusicalSequence {
    fn new() -> Self {
        MusicalSequence {
            upper_y: 1,
            ..Default::default()
        }
    }

    pub(crate) fn new_with_coords(x: f64, y: f64, r: f64) -> Self {
        MusicalSequence {
            upper_y: 1,
            center_x: x,
            center_y: y,
            rotation: r,
            ..Default::default()
        }
    }

    pub(crate) fn set_zeroeth(&mut self, distance: f64) {
        self.center_x = distance * self.rotation.cos();
        self.center_y = distance * self.rotation.sin();
    }

    fn find_upper_intercept(&self) -> f64 {
        self.upper_y as f64 - (self.upper_x as f64 * golden_ratio::<f64>())
    }

    fn find_lower_intercept(&self) -> f64 {
        self.lower_y as f64 - (self.lower_x as f64 * golden_ratio::<f64>())
    }

    fn find_point(&self, x: BarNumber, bound: BarBound) -> f64 {
        let intercept = if bound == BarBound::Longer {
            self.find_upper_intercept()
        } else {
            self.find_lower_intercept()
        };
        let res = x as f64 * golden_ratio::<f64>() + intercept;
        res
    }

    fn find_upper_point(&self, bar: BarNumber) -> BarNumber {
        truncate_closed(self.find_point(bar, BarBound::Longer))
    }

    fn find_lower_point(&self, bar: BarNumber) -> BarNumber {
        truncate_open(self.find_point(bar, BarBound::Shorter))
    }

    pub fn force(&mut self, bar: BarNumber, bound: BarBound) {
        let longer = self.find_upper_point(bar);
        let shorter = self.find_lower_point(bar);

        if longer != shorter {
            if (bound == BarBound::Longer && bar >= 0) || (bound == BarBound::Shorter && bar < 0) {
                self.lower_x = bar;
                self.lower_y = longer;
            } else {
                self.upper_x = bar;
                self.upper_y = longer;
            }
        }
    }

    pub(crate) fn is_forced(&self, bar: BarNumber) -> bool {
        self.find_upper_point(bar) == self.find_lower_point(bar)
    }

    pub(crate) fn get_bar_distance(&self, bar: BarNumber) -> f64 {
        let y = self.find_upper_point(bar);
        let shorts = (2 * bar) - y;
        let longs = y - bar;

        scale::<f64>() * (shorts as f64 + golden_ratio::<f64>() * longs as f64)
    }

    pub(crate) fn get_bar(&self, distance: f64) -> BarNumber {
        let shorts = distance / (short::<f64>() + (golden_ratio::<f64>() * long::<f64>()));
        let longs = shorts * golden_ratio::<f64>();

        let sum = shorts + longs;
        let rem = sum.rem_euclid(1f64);

        // java round is different to rust round!
        if rem >= 0.5 {
            (sum - rem + 1f64).to_i64().unwrap()
        } else {
            (sum - rem).to_i64().unwrap()
        }
    }

    pub(crate) fn force_at_distance(&mut self, distance: f64) -> bool {
        let scaled = distance / scale::<f64>();

        let shorts = scaled / (2f64 + golden_ratio::<f64>());
        let longs = shorts * golden_ratio::<f64>();

        let shorts = {
            let first = shorts.floor() as BarNumber;
            [first, first + 1]
        };
        let longs = {
            let first = longs.floor() as BarNumber;
            [first, first + 1]
        };

        let (best_short, best_long) = shorts
            .iter()
            .copied()
            .enumerate()
            .cartesian_product(longs.iter().copied().enumerate())
            .map(|((i, shorts), (j, longs))| (i, j, ddist(shorts, longs, scaled)))
            .min_by(|(_, _, ddist1), (_, _, ddist2)| ddist1.partial_cmp(ddist2).unwrap_or(Equal))
            .map(|(i, j, _)| (i, j))
            .unwrap();

        let bars = shorts[best_short] + longs[best_long];

        if !self.is_forced(bars) {
            if (best_long == 1 && bars > -1) || (best_long == 0 && bars < 0) {
                self.force(bars, BarBound::Longer);
            } else {
                self.force(bars, BarBound::Shorter);
            }
            true
        } else {
            false
        }
    }

    pub fn get_bar_forcings<I>(&self, r: I) -> Vec<bool>
    where
        I: Iterator<Item = BarNumber>,
    {
        r.map(|i| self.is_forced(i)).collect_vec()
    }

    pub fn guess_bars(&self, r: Range<BarNumber>) -> Vec<BarBound> {
        let mut res = Vec::new();
        let mut last_high = 0;
        for i in r.map(|i| i + 1) {
            let shorter = self.find_lower_point(i);

            if shorter - last_high == 1 {
                res.push(BarBound::Shorter);
            } else {
                res.push(BarBound::Longer);
            }

            last_high = shorter;
        }
        res
    }

    pub(crate) fn center_x(&self) -> f64 {
        self.center_x
    }

    pub(crate) fn center_y(&self) -> f64 {
        self.center_y
    }

    pub(crate) fn rotation(&self) -> f64 {
        self.rotation
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test_force<const N: usize>(
        ammann: MusicalSequence,
        r: Range<BarNumber>,
        bounds: [BarBound; N],
        forces: [bool; N],
        upper_x: BarNumber,
        upper_y: BarNumber,
        lower_x: BarNumber,
        lower_y: BarNumber,
    ) {
        for (i, (expected, actual)) in r.clone().zip(
            bounds.iter().copied().zip(forces).zip(
                ammann
                    .guess_bars(r.clone())
                    .iter()
                    .copied()
                    .zip(ammann.get_bar_forcings(r.map(|i| i + 1))),
            ),
        ) {
            assert_eq!(
                expected, actual,
                "bound {} expected to be {:?}, found {:?}",
                i, expected, actual
            );
        }
        assert_eq!(upper_x, ammann.upper_x);
        assert_eq!(upper_y, ammann.upper_y);
        assert_eq!(lower_x, ammann.lower_x);
        assert_eq!(lower_y, ammann.lower_y);
    }

    #[test]
    fn force_at_distance_short() {
        let mut ammann = MusicalSequence::new();

        ammann.force_at_distance(short::<f64>());
        test_force(
            ammann,
            0..3,
            [BarBound::Shorter, BarBound::Longer, BarBound::Shorter],
            [true, true, false],
            1,
            2,
            0,
            0,
        );
    }

    #[test]
    fn force_shorter() {
        let mut ammann = MusicalSequence::new();

        ammann.force(1, BarBound::Shorter);
        test_force(
            ammann,
            0..3,
            [BarBound::Shorter, BarBound::Longer, BarBound::Shorter],
            [true, true, false],
            1,
            2,
            0,
            0,
        );
    }

    #[test]
    fn force_at_distance_long() {
        let mut ammann = MusicalSequence::new();

        ammann.force_at_distance(long::<f64>());
        test_force(
            ammann,
            0..3,
            [BarBound::Longer, BarBound::Shorter, BarBound::Longer],
            [true, false, true],
            0,
            1,
            1,
            2,
        );
    }

    #[test]
    fn force_longer() {
        let mut ammann = MusicalSequence::new();

        ammann.force(1, BarBound::Longer);
        test_force(
            ammann,
            0..3,
            [BarBound::Longer, BarBound::Shorter, BarBound::Longer],
            [true, false, true],
            0,
            1,
            1,
            2,
        );
    }

    #[test]
    fn force_at_distance_negative_short() {
        let mut ammann = MusicalSequence::new();

        ammann.force_at_distance(-short::<f64>());
        test_force(
            ammann,
            -2..1,
            [BarBound::Longer, BarBound::Shorter, BarBound::Longer],
            [true, true, true],
            0,
            1,
            -1,
            -1,
        )
    }

    #[test]
    fn force_negative_shorter() {
        let mut ammann = MusicalSequence::new();

        ammann.force(-1, BarBound::Shorter);
        test_force(
            ammann,
            -2..1,
            [BarBound::Longer, BarBound::Shorter, BarBound::Longer],
            [true, true, true],
            0,
            1,
            -1,
            -1,
        )
    }

    #[test]
    fn force_at_distance_negative_long() {
        let mut ammann = MusicalSequence::new();

        ammann.force_at_distance(-long::<f64>());
        test_force(
            ammann,
            -2..1,
            [BarBound::Longer, BarBound::Longer, BarBound::Shorter],
            [true, true, false],
            -1,
            -1,
            0,
            0,
        )
    }

    #[test]
    fn force_negative_longer() {
        let mut ammann = MusicalSequence::new();

        ammann.force(-1, BarBound::Longer);
        test_force(
            ammann,
            -2..1,
            [BarBound::Longer, BarBound::Longer, BarBound::Shorter],
            [true, true, false],
            -1,
            -1,
            0,
            0,
        )
    }
}
