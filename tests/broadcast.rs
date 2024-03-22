use trybuild::TestCases;

#[test]
fn test() {
    let t = TestCases::new();

    t.pass("tests/broadcast/array1_pass.rs");
    t.compile_fail("tests/broadcast/array1_fail.rs");

    t.pass("tests/broadcast/array2_pass.rs");
    t.compile_fail("tests/broadcast/array2_fail.rs");

    t.pass("tests/broadcast/array3_pass.rs");
    t.compile_fail("tests/broadcast/array3_fail.rs");
}
