use trybuild::TestCases;

#[test]
fn test() {
    let t = TestCases::new();
    t.pass("tests/broadcast/array1_scalar.rs");
}
