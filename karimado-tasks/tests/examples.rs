#[test]
fn examples() {
    trycmd::TestCases::new()
        .register_bins(trycmd::cargo::compile_examples([]).unwrap())
        .case("examples/taskmgr.md")
        .case("examples/taskmgr-*.md");
}
