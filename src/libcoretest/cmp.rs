// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use core::cmp::{partial_min, partial_max};
use core::cmp::Ordering::{Less, Greater, Equal};

#[test]
fn test_int_totalord() {
    assert_eq!(5.cmp(&10), Less);
    assert_eq!(10.cmp(&5), Greater);
    assert_eq!(5.cmp(&5), Equal);
    assert_eq!((-5).cmp(&12), Less);
    assert_eq!(12.cmp(&-5), Greater);
}

#[test]
fn test_mut_int_totalord() {
    assert_eq!((&mut 5).cmp(&&mut 10), Less);
    assert_eq!((&mut 10).cmp(&&mut 5), Greater);
    assert_eq!((&mut 5).cmp(&&mut 5), Equal);
    assert_eq!((&mut -5).cmp(&&mut 12), Less);
    assert_eq!((&mut 12).cmp(&&mut -5), Greater);
}

#[test]
fn test_ordering_reverse() {
    assert_eq!(Less.reverse(), Greater);
    assert_eq!(Equal.reverse(), Equal);
    assert_eq!(Greater.reverse(), Less);
}

#[test]
fn test_ordering_order() {
    assert!(Less < Equal);
    assert_eq!(Greater.cmp(&Less), Greater);
}

#[test]
fn test_partial_min() {
    use core::f64::NAN;
    let data_integer = [
        // a, b, result
        (0, 0, Some(0)),
        (1, 0, Some(0)),
        (0, 1, Some(0)),
        (-1, 0, Some(-1)),
        (0, -1, Some(-1))
    ];

    let data_float = [
        // a, b, result
        (0.0f64, 0.0f64, Some(0.0f64)),
        (1.0f64, 0.0f64, Some(0.0f64)),
        (0.0f64, 1.0f64, Some(0.0f64)),
        (-1.0f64, 0.0f64, Some(-1.0f64)),
        (0.0f64, -1.0f64, Some(-1.0f64)),
        (NAN, NAN, None),
        (NAN, 1.0f64, None),
        (1.0f64, NAN, None)
    ];

    for &(a, b, result) in &data_integer {
        assert!(partial_min(a, b) == result);
    }

    for &(a, b, result) in &data_float {
        assert!(partial_min(a, b) == result);
    }
}

#[test]
fn test_partial_max() {
    use core::f64::NAN;
    let data_integer = [
        // a, b, result
        (0, 0, Some(0)),
        (1, 0, Some(1)),
        (0, 1, Some(1)),
        (-1, 0, Some(0)),
        (0, -1, Some(0))
    ];

    let data_float = [
        // a, b, result
        (0.0f64, 0.0f64, Some(0.0f64)),
        (1.0f64, 0.0f64, Some(1.0f64)),
        (0.0f64, 1.0f64, Some(1.0f64)),
        (-1.0f64, 0.0f64, Some(0.0f64)),
        (0.0f64, -1.0f64, Some(0.0f64)),
        (NAN, NAN, None),
        (NAN, 1.0f64, None),
        (1.0f64, NAN, None)
    ];

    for &(a, b, result) in &data_integer {
        assert!(partial_max(a, b) == result);
    }

    for &(a, b, result) in &data_float {
        assert!(partial_max(a, b) == result);
    }
}

#[test]
fn test_user_defined_eq() {
    use core::num::SignedInt;

    // Our type.
    struct SketchyNum {
        num : isize
    }

    // Our implementation of `PartialEq` to support `==` and `!=`.
    impl PartialEq for SketchyNum {
        // Our custom eq allows numbers which are near each other to be equal! :D
        fn eq(&self, other: &SketchyNum) -> bool {
            (self.num - other.num).abs() < 5
        }
    }

    // Now these binary operators will work when applied!
    assert!(SketchyNum {num: 37} == SketchyNum {num: 34});
    assert!(SketchyNum {num: 25} != SketchyNum {num: 57});
}
