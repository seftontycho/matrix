#![feature(generic_const_exprs)]

use matrix::prelude::*;

fn main() {
    // impossible shape combinations are:
    // [A], [B] where A != B and A != 1 and B != 1

    array1_array1_unequal();
}

/// This is case: [A], [B] where A != B and A != 1 and B != 1
fn array1_array1_unequal() {
    let a = Array1::from([1, 2, 3]);
    let b = Array1::from([1, 2]);
    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);
}
