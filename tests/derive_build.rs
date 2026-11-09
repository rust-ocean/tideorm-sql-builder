#[test]
fn build_tests() {
    let t = trybuild::TestCases::new();
    t.compile_fail("./tests/derive/compile-fail/*.rs");

    t.pass("./tests/derive/pass/*.rs");
    t.pass("./tests/derive/pass-static/*.rs");
}
