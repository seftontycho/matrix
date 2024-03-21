use matrix::prelude::*;

fn main() {
    let a = Array1::<i32, 3>::from([1, 2, 3]);
    let b = 2;

    let c = a.broadcast(b, |a, b| a * b);

    assert_eq!(c, Array1::from([2, 4, 6]));
}
