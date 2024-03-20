#[ctor::ctor]
fn global_setup() {
    env_logger::builder().is_test(true).init();
}
