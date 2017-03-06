// Copyright 2013-2014 The acgmath Developers. For a full listing of the authors,
// refer to the Cargo.toml file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_use]
extern crate approx;
extern crate acgmath;

use acgmath::{Rad, Deg};

#[test]
fn conv() {
    let angle: Rad<_> = Deg(-5.0f64).into();
    let angle: Deg<_> = angle.into();
    assert_ulps_eq!(&angle, &Deg(-5.0f64));

    let angle: Rad<_> = Deg(30.0f64).into();
    let angle: Deg<_> = angle.into();
    assert_ulps_eq!(&angle, &Deg(30.0f64));

    let angle: Deg<_> = Rad(-5.0f64).into();
    let angle: Rad<_> = angle.into();
    assert_ulps_eq!(&angle, &Rad(-5.0f64));

    let angle: Deg<_> = Rad(30.0f64).into();
    let angle: Rad<_> = angle.into();
    assert_ulps_eq!(&angle, &Rad(30.0f64));
}
