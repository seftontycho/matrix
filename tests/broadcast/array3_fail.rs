#![feature(generic_const_exprs)]

use matrix::prelude::*;

fn main() {
    // Array1
    // impossible shape combinations are:
    // [A, B, C], [D] where C != D and C, D != 1

    a_b_c_d();

    // Array2
    // impossible shape combinations are:
    // [A, B, C], [D, C] where B != D and B, D != 1
    // [A, B, C], [B, D] where C != D and C, D != 1

    a_b_c_d_c();
    a_b_c_b_d();

    // Array3
    // impossible shape combinations are:
    // [A, B, C], [D, B, C] where A != D and A, D != 1
    // [A, B, C], [A, D, C] where B != D and B, D != 1
    // [A, B, C], [A, B, D] where C != D and C, D != 1

    a_b_c_d_b_c();
    a_b_c_a_d_c();
    a_b_c_a_b_d();
}

/// This is case: [A, B, C], [D] where C != D and C, D != 1
fn a_b_c_d() {
    // shape: [2, 3, 4]
    let a = Array3::from([
        [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]],
        [[13, 14, 15, 16], [17, 18, 19, 20], [21, 22, 23, 24]],
    ]);

    // shape: [5]
    let b = Array1::from([1, 2, 3, 4, 5]);

    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);
}

/// This is case: [A, B, C], [D, C] where B != D and B, D != 1
fn a_b_c_d_c() {
    // shape: [2, 3, 4]
    let a = Array3::from([
        [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]],
        [[13, 14, 15, 16], [17, 18, 19, 20], [21, 22, 23, 24]],
    ]);

    // shape: [5, 4]
    let b = Array2::from([
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 10, 11, 12],
        [13, 14, 15, 16],
        [17, 18, 19, 20],
    ]);

    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);
}

/// This is case: [A, B, C], [B, D] where C != D and C, D != 1
fn a_b_c_b_d() {
    // shape: [2, 3, 4]
    let a = Array3::from([
        [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]],
        [[13, 14, 15, 16], [17, 18, 19, 20], [21, 22, 23, 24]],
    ]);

    // shape: [3, 5]
    let b = Array2::from([[1, 2, 3, 4, 5], [6, 7, 8, 9, 10], [11, 12, 13, 14, 15]]);

    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);
}

/// This is case: [A, B, C], [D, B, C] where A != D and A, D != 1
fn a_b_c_d_b_c() {
    // shape: [2, 3, 4]
    let a = Array3::from([
        [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]],
        [[13, 14, 15, 16], [17, 18, 19, 20], [21, 22, 23, 24]],
    ]);

    // shape: [5, 3, 4]
    let b = Array3::from([
        [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]],
        [[13, 14, 15, 16], [17, 18, 19, 20], [21, 22, 23, 24]],
        [[25, 26, 27, 28], [29, 30, 31, 32], [33, 34, 35, 36]],
        [[37, 38, 39, 40], [41, 42, 43, 44], [45, 46, 47, 48]],
        [[49, 50, 51, 52], [53, 54, 55, 56], [57, 58, 59, 60]],
    ]);

    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);
}

/// This is case: [A, B, C], [A, D, C] where B != D and B, D != 1
fn a_b_c_a_d_c() {
    // shape: [2, 3, 4]
    let a = Array3::from([
        [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]],
        [[13, 14, 15, 16], [17, 18, 19, 20], [21, 22, 23, 24]],
    ]);

    // shape: [2, 5, 4]
    let b = Array3::from([
        [
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
            [17, 18, 19, 20],
        ],
        [
            [21, 22, 23, 24],
            [25, 26, 27, 28],
            [29, 30, 31, 32],
            [33, 34, 35, 36],
            [37, 38, 39, 40],
        ],
    ]);

    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);
}

/// This is case: [A, B, C], [A, B, D] where C != D and C, D != 1
fn a_b_c_a_b_d() {
    // shape: [2, 3, 4]
    let a = Array3::from([
        [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]],
        [[13, 14, 15, 16], [17, 18, 19, 20], [21, 22, 23, 24]],
    ]);

    // shape: [2, 3, 5]
    let b = Array3::from([
        [[1, 2, 3, 4, 5], [6, 7, 8, 9, 10], [11, 12, 13, 14, 15]],
        [
            [16, 17, 18, 19, 20],
            [21, 22, 23, 24, 25],
            [26, 27, 28, 29, 30],
        ],
    ]);

    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);
}
