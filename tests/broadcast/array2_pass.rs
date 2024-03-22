#![feature(generic_const_exprs)]

use matrix::prelude::*;

fn main() {
    array2_scalar();

    // Array1
    // possible shape combinations are:
    // [1, A], [A] -> [1, A]
    // [A, 1], [B] -> [A, B]
    // [A, B], [B] -> [A, B] where B != 1

    array2_array1_1st_dim_1();
    array2_array1_2nd_dim_1();
    array2_array1_equal();

    // Array2
    // possible shape combinations are:
    // [1, A], [B, A] -> [B, A]
    // [A, 1], [A, B] -> [A, B]
    // [A, B], [1, B] -> [A, B]
    // [A, B], [A, 1] -> [A, B]
    // [A, B], [A, B] -> [A, B] where A, B != 1

    one_a_b_a();
    a_one_a_b();
    a_b_one_b();
    a_b_a_one();
    a_b_a_b();
}

fn array2_scalar() {
    let a = Array2::from([[1, 2, 3], [3, 4, 5]]);
    let b = 2;
    let op = |a: &i32, b: &i32| a * b;

    let c = a.broadcast(b, |a, b| a * b);

    assert_eq!(c, Array2::from([[2, 4, 6], [6, 8, 10]]));
}

/// This is case: [1, A], [A] -> [1, A]
fn array2_array1_1st_dim_1() {
    let a = Array2::from([[1, 2, 3]]);
    let b = Array1::from([1, 2, 3]);
    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    assert_eq!(c, Array2::from([[1, 4, 9]]));
}

/// This is case: [A, 1], [B] -> [A, B]
fn array2_array1_2nd_dim_1() {
    let a = Array2::from([[1], [2], [3]]);
    let b = Array1::from([1, 2, 3]);
    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    assert_eq!(c, Array2::from([[1, 2, 3], [2, 4, 6], [3, 6, 9]]));
}

/// This is case: [A, B], [B] -> [A, B] where B != 1
fn array2_array1_equal() {
    let a = Array2::from([[1, 2, 3], [4, 5, 6]]);
    let b = Array1::from([1, 2, 3]);
    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    assert_eq!(c, Array2::from([[1, 4, 9], [4, 10, 18]]));
}

/// This is case: [1, A], [B, A] -> [B, A]
fn one_a_b_a() {
    let a = Array2::from([[1, 2, 3]]);
    let b = Array2::from([[1, 2, 3], [4, 5, 6]]);
    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    assert_eq!(c, Array2::from([[1, 4, 9], [4, 10, 18]]));
}

/// This is case: [A, 1], [A, B] -> [A, B]
fn a_one_a_b() {
    let a = Array2::from([[1], [2], [3]]);
    let b = Array2::from([[1, 2], [3, 4], [5, 6]]);
    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    assert_eq!(c, Array2::from([[1, 2], [6, 8], [15, 18]]));
}

/// This is case: [A, B], [1, B] -> [A, B]
fn a_b_one_b() {
    let a = Array2::from([[1, 2, 3], [4, 5, 6]]);
    let b = Array2::from([[1, 2, 3]]);
    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    assert_eq!(c, Array2::from([[1, 4, 9], [4, 10, 18]]));
}

/// This is case: [A, B], [A, 1] -> [A, B]
fn a_b_a_one() {
    let a = Array2::from([[1, 2, 3], [4, 5, 6]]);
    let b = Array2::from([[1], [2]]);
    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    assert_eq!(c, Array2::from([[1, 2, 3], [8, 10, 12]]));
}

/// This is case: [A, B], [A, B] -> [A, B] where A, B != 1
fn a_b_a_b() {
    let a = Array2::from([[1, 2, 3], [4, 5, 6]]);
    let b = Array2::from([[1, 2, 3], [4, 5, 6]]);
    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    assert_eq!(c, Array2::from([[1, 4, 9], [16, 25, 36]]));
}
