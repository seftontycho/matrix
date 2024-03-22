#![feature(generic_const_exprs)]

use matrix::prelude::*;

fn main() {
    // impossible shape combinations are:
    // [A, B], [C, B] where A != C and A != 1 and C != 1
    // [A, B], [A, C] where B != C and B != 1 and C != 1
    // [A, B], [C, D] where A != C and B != C and A, B, C, D != 1

    a_b_c_b();
    a_b_a_c();
    a_b_c_d();
}

/// This is case: [A, B], [C, B] where A != C and A != 1 and C != 1
fn a_b_c_b() {
    let a = Array2::from([[1, 2], [3, 4], [5, 6]]);
    let b = Array2::from([[1, 2], [3, 4]]);
    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);
}

/// This is case: [A, B], [A, C] where B != C and B != 1 and C != 1
fn a_b_a_c() {
    let a = Array2::from([[1, 2], [3, 4]]);
    let b = Array2::from([[1, 2, 3], [4, 5, 6]]);
    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);
}

/// This is case: [A, B], [C, D] where A != C and B != C and A, B, C, D != 1
fn a_b_c_d() {
    let a = Array2::from([[1, 2, 3], [4, 5, 6]]); // [2, 3]
    let b = Array2::from([[1, 2], [3, 4], [5, 6]]); // [3, 2]
    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);
}
