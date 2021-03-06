// Anima Engine. The quirky game engine
// Copyright (C) 2016  Dragoș Tiselice
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

extern crate anima;

type Quaternion = anima::math::Quaternion;

#[test]
fn test_mul() {
    let q1 = Quaternion::new(0.0, 1.0, 2.0, 3.0);
    let q2 = Quaternion::new(3.0, 2.0, 1.0, 0.0);

    assert_eq!(q1 * q2, Quaternion::new(12.0, 0.0, 6.0, -4.0));
}
