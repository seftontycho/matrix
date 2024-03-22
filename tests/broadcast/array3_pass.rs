#![feature(generic_const_exprs)]

use matrix::prelude::*;

fn main() {
    array3_scalar();

    // Array1
    //possible shape combinations are:
    // [A, B, 1], [C] -> [A, B, C]
    // [A, B, C], [1] -> [A, B, C]
    //
    // [A, B, C], [C] -> [A, B, C] where C != 1

    a_b_one_c();
    a_b_c_one();
    a_b_c_c();

    // Array2
    // possible shape combinations are:
    // [1, A, B], [A, B] -> [1, A, B]
    // [A, 1, B], [C, B] -> [A, C, B]
    // [A, B, 1], [B, C] -> [A, B, C]
    //
    // [A, B, C], [1, C] -> [A, B, C]
    // [A, B, C], [B, 1] -> [A, B, C]
    //
    // [A, B, C], [B, C] -> [A, B, C] where B, C != 1

    one_a_b_a_b();
    a_one_b_c_b();
    a_b_one_b_c();

    a_b_c_one_c();
    a_b_c_b_one();

    a_b_c_b_c();

    // Array3
    // possible shape combinations are:
    // [1, A, B], [C, A, B] -> [C, A, B]
    // [A, 1, B], [A, C, B] -> [A, C, B]
    // [A, B, 1], [A, B, C] -> [A, B, C]
    //
    // [A, B, C], [1, B, C] -> [A, B, C]
    // [A, B, C], [A, 1, C] -> [A, B, C]
    // [A, B, C], [A, B, 1] -> [A, B, C]
    //
    // [A, B, C], [A, B, C] -> [A, B, C] where A, B, C != 1

    one_a_b_c_a_b();
    a_one_b_a_c_b();
    a_b_one_a_b_c();

    a_b_c_one_b_c();
    a_b_c_a_one_c();
    a_b_c_a_b_one();

    a_b_c_a_b_c();
}

fn array3_scalar() {
    let a = Array3::from([[[1, 2], [3, 4]], [[5, 6], [7, 8]]]);
    let b = 2;
    let op = |a: &i32, b: &i32| a * b;

    let c = a.broadcast(b, |a, b| a * b);

    assert_eq!(c, Array3::from([[[2, 4], [6, 8]], [[10, 12], [14, 16]]]));
}

/// This is case: [A, B, 1], [C] -> [A, B, C]
fn a_b_one_c() {
    // shape: [2, 3, 1]
    let a = Array3::from([[[1], [2], [3]], [[4], [5], [6]]]);
    let b = Array1::from([1, 2, 3, 4]); // shape: [4]
    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    // shape: [2, 3, 4]
    let expected = Array3::from([
        [[1, 2, 3, 4], [2, 4, 6, 8], [3, 6, 9, 12]],
        [[4, 8, 12, 16], [5, 10, 15, 20], [6, 12, 18, 24]],
    ]);

    assert_eq!(c, expected);
}

/// This is case: [A, B, C], [1] -> [A, B, C]
fn a_b_c_one() {
    // shape: [2, 3, 4]
    let a = Array3::from([
        [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]],
        [[13, 14, 15, 16], [17, 18, 19, 20], [21, 22, 23, 24]],
    ]);
    let b = Array1::from([1]); // shape: [1]
    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    // shape: [2, 3, 4]
    let expected = Array3::from([
        [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]],
        [[13, 14, 15, 16], [17, 18, 19, 20], [21, 22, 23, 24]],
    ]);

    assert_eq!(c, expected);
}

/// This is case: [A, B, C], [C] -> [A, B, C] where C != 1
fn a_b_c_c() {
    // shape: [2, 3, 4]
    let a = Array3::from([
        [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]],
        [[13, 14, 15, 16], [17, 18, 19, 20], [21, 22, 23, 24]],
    ]);
    let b = Array1::from([1, 2, 3, 4]); // shape: [4]
    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    // shape: [2, 3, 4]
    let expected = Array3::from([
        [[1, 4, 9, 16], [5, 12, 21, 32], [9, 20, 33, 48]],
        [[13, 28, 45, 64], [17, 36, 57, 80], [21, 44, 69, 96]],
    ]);

    assert_eq!(c, expected);
}

/// This is case: [1, A, B], [A, B] -> [1, A, B]
fn one_a_b_a_b() {
    let a = Array3::from([[[1, 2, 3], [4, 5, 6]]]); // shape: [1, 2, 3]
    let b = Array2::from([[1, 2, 3], [4, 5, 6]]); // shape: [2, 3]
    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    assert_eq!(c, Array3::from([[[1, 4, 9], [16, 25, 36]]]));
}

/// This is case: [A, 1, B], [C, B] -> [A, C, B]
fn a_one_b_c_b() {
    // shape: [2, 1, 3]
    let a = Array3::from([[[1, 2, 3]], [[4, 5, 6]]]);

    // shape: [4, 3]
    let b = Array2::from([[1, 2, 3], [4, 5, 6], [7, 8, 9], [10, 11, 12]]);
    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    // shape: [2, 4, 3]
    let expected = Array3::from([
        [[1, 4, 9], [4, 10, 18], [7, 16, 27], [10, 22, 36]],
        [[4, 10, 18], [16, 25, 36], [28, 40, 54], [40, 55, 72]],
    ]);

    assert_eq!(c, expected);
}

/// This is case: [A, B, 1], [B, C] -> [A, B, C]
fn a_b_one_b_c() {
    // shape: [2, 3, 1]
    let a = Array3::from([[[1], [2], [3]], [[4], [5], [6]]]);

    // shape: [3, 4]
    let b = Array2::from([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]);
    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    // shape: [2, 3, 4]
    let expected = Array3::from([
        [[1, 2, 3, 4], [10, 12, 14, 16], [27, 30, 33, 36]],
        [[4, 8, 12, 16], [25, 30, 35, 40], [54, 60, 66, 72]],
    ]);

    assert_eq!(c, expected);
}

/// This is case: [A, B, C], [1, C] -> [A, B, C]
fn a_b_c_one_c() {
    // shape: [2, 3, 4]
    let a = Array3::from([
        [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]],
        [[13, 14, 15, 16], [17, 18, 19, 20], [21, 22, 23, 24]],
    ]);

    // shape: [1, 4]
    let b = Array2::from([[1, 2, 3, 4]]);
    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    // shape: [2, 3, 4]
    let expected = Array3::from([
        [[1, 4, 9, 16], [5, 12, 21, 32], [9, 20, 33, 48]],
        [[13, 28, 45, 64], [17, 36, 57, 80], [21, 44, 69, 96]],
    ]);

    assert_eq!(c, expected);
}

/// This is case: [A, B, C], [B, 1] -> [A, B, C]
fn a_b_c_b_one() {
    // shape: [2, 3, 4]
    let a = Array3::from([
        [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]],
        [[13, 14, 15, 16], [17, 18, 19, 20], [21, 22, 23, 24]],
    ]);

    // shape: [3, 1]
    let b = Array2::from([[1], [2], [3]]);
    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    // shape: [2, 3, 4]
    let expected = Array3::from([
        [[1, 2, 3, 4], [10, 12, 14, 16], [27, 30, 33, 36]],
        [[13, 14, 15, 16], [34, 36, 38, 40], [63, 66, 69, 72]],
    ]);

    assert_eq!(c, expected);
}

/// This is case: [A, B, C], [B, C] -> [A, B, C] where B, C != 1
fn a_b_c_b_c() {
    // shape: [2, 3, 4]
    let a = Array3::from([
        [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]],
        [[13, 14, 15, 16], [17, 18, 19, 20], [21, 22, 23, 24]],
    ]);

    // shape: [3, 4]
    let b = Array2::from([[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]);
    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    // shape: [2, 3, 4]
    let expected = Array3::from([
        [[1, 4, 9, 16], [25, 36, 49, 64], [81, 100, 121, 144]],
        [[13, 28, 45, 64], [85, 108, 133, 160], [189, 220, 253, 288]],
    ]);

    assert_eq!(c, expected);
}

/// This is case: [1, A, B], [C, A, B] -> [C, A, B]
fn one_a_b_c_a_b() {
    // shape: [1, 2, 3]
    let a = Array3::from([[[1, 2, 3], [4, 5, 6]]]);

    // shape: [4, 2, 3]
    let b = Array3::from([
        [[1, 2, 3], [4, 5, 6]],
        [[7, 8, 9], [10, 11, 12]],
        [[13, 14, 15], [16, 17, 18]],
        [[19, 20, 21], [22, 23, 24]],
    ]);

    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    // shape: [4, 2, 3]
    let expected = Array3::from([
        [[1, 4, 9], [16, 25, 36]],
        [[7, 16, 27], [40, 55, 72]],
        [[13, 28, 45], [64, 85, 108]],
        [[19, 40, 63], [88, 115, 144]],
    ]);

    assert_eq!(c, expected);
}

// This is case: [A, 1, B], [A, C, B] -> [A, C, B]
fn a_one_b_a_c_b() {
    // shape: [2, 1, 3]
    let a = Array3::from([[[1, 2, 3]], [[4, 5, 6]]]);

    // shape: [2, 4, 3]
    let b = Array3::from([
        [[1, 2, 3], [4, 5, 6], [7, 8, 9], [10, 11, 12]],
        [[13, 14, 15], [16, 17, 18], [19, 20, 21], [22, 23, 24]],
    ]);

    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    // shape: [2, 4, 3]
    let expected = Array3::from([
        [[1, 4, 9], [4, 10, 18], [7, 16, 27], [10, 22, 36]],
        [[52, 70, 90], [64, 85, 108], [76, 100, 126], [88, 115, 144]],
    ]);

    assert_eq!(c, expected);
}

// This is case: [A, B, 1], [A, B, C] -> [A, B, C]
fn a_b_one_a_b_c() {
    // shape: [2, 3, 1]
    let a = Array3::from([[[1], [2], [3]], [[4], [5], [6]]]);

    // shape: [2, 3, 4]
    let b = Array3::from([
        [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]],
        [[13, 14, 15, 16], [17, 18, 19, 20], [21, 22, 23, 24]],
    ]);

    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    // shape: [2, 3, 4]
    let expected = Array3::from([
        [[1, 2, 3, 4], [10, 12, 14, 16], [27, 30, 33, 36]],
        [[52, 56, 60, 64], [85, 90, 95, 100], [126, 132, 138, 144]],
    ]);

    assert_eq!(c, expected);
}

// This is case: [A, B, C], [1, B, C] -> [A, B, C]
fn a_b_c_one_b_c() {
    // shape: [2, 3, 4]
    let a = Array3::from([
        [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]],
        [[13, 14, 15, 16], [17, 18, 19, 20], [21, 22, 23, 24]],
    ]);

    // shape: [1, 3, 4]
    let b = Array3::from([[[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]]]);

    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    // shape: [2, 3, 4]
    let expected = Array3::from([
        [[1, 4, 9, 16], [25, 36, 49, 64], [81, 100, 121, 144]],
        [[13, 28, 45, 64], [85, 108, 133, 160], [189, 220, 253, 288]],
    ]);

    assert_eq!(c, expected);
}

// This is case: [A, B, C], [A, 1, C] -> [A, B, C]
fn a_b_c_a_one_c() {
    // shape: [2, 3, 4]
    let a = Array3::from([
        [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]],
        [[13, 14, 15, 16], [17, 18, 19, 20], [21, 22, 23, 24]],
    ]);

    // shape: [2, 1, 4]
    let b = Array3::from([[[1, 2, 3, 4]], [[5, 6, 7, 8]]]);

    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    // shape: [2, 3, 4]
    let expected = Array3::from([
        [[1, 4, 9, 16], [5, 12, 21, 32], [9, 20, 33, 48]],
        [
            [65, 84, 105, 128],
            [85, 108, 133, 160],
            [105, 132, 161, 192],
        ],
    ]);

    assert_eq!(c, expected);
}

// This is case: [A, B, C], [A, B, 1] -> [A, B, C]
fn a_b_c_a_b_one() {
    // shape: [2, 3, 4]
    let a = Array3::from([
        [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]],
        [[13, 14, 15, 16], [17, 18, 19, 20], [21, 22, 23, 24]],
    ]);

    // shape: [2, 3, 1]
    let b = Array3::from([[[1], [2], [3]], [[4], [5], [6]]]);

    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    // shape: [2, 3, 4]
    let expected = Array3::from([
        [[1, 2, 3, 4], [10, 12, 14, 16], [27, 30, 33, 36]],
        [[52, 56, 60, 64], [85, 90, 95, 100], [126, 132, 138, 144]],
    ]);

    assert_eq!(c, expected);
}

// This is case: [A, B, C], [A, B, C] -> [A, B, C] where A, B, C != 1
fn a_b_c_a_b_c() {
    // shape: [2, 3, 4]
    let a = Array3::from([
        [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]],
        [[13, 14, 15, 16], [17, 18, 19, 20], [21, 22, 23, 24]],
    ]);

    // shape: [2, 3, 4]
    let b = Array3::from([
        [[1, 2, 3, 4], [5, 6, 7, 8], [9, 10, 11, 12]],
        [[13, 14, 15, 16], [17, 18, 19, 20], [21, 22, 23, 24]],
    ]);

    let op: fn(&i32, &i32) -> i32 = |a, b| a * b;

    let c = a.broadcast(b, op);

    // shape: [2, 3, 4]
    let expected = Array3::from([
        [[1, 4, 9, 16], [25, 36, 49, 64], [81, 100, 121, 144]],
        [
            [169, 196, 225, 256],
            [289, 324, 361, 400],
            [441, 484, 529, 576],
        ],
    ]);

    assert_eq!(c, expected);
}
