#![feature(generic_const_exprs)]

use matrix::prelude::*;

fn main() {
    array1_scalar();

    // Array1
    // possible shape combinations are:
    // [1], [A] -> [A]
    // [A], [1] -> [A]
    // [A], [A] -> [A]

    array1_array1_1st_is_1();
    array1_array1_2nd_is_1();
    array1_array1_equal();
}

fn array1_scalar() {
    let a = Array1::from([1, 2, 3]);
    let b = 2;
    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    assert_eq!(c, Array1::from([2, 4, 6]));
}

/// This is case: [A], [A] -> [A]
fn array1_array1_equal() {
    let a = Array1::from([1, 2, 3]);
    let b = Array1::from([1, 2, 3]);
    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    assert_eq!(c, Array1::from([1, 4, 9]));
}

/// This is case: [A], [1] -> [A]
fn array1_array1_2nd_is_1() {
    let a = Array1::from([1, 2, 3]);
    let b = Array1::from([1]);
    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    assert_eq!(c, Array1::from([1, 2, 3]));
}

/// This is case: [1], [A] -> [A]
fn array1_array1_1st_is_1() {
    let a = Array1::from([1]);
    let b = Array1::from([1, 2, 3]);
    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    assert_eq!(c, Array1::from([1, 2, 3]));
}
