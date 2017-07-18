extern crate balertest;
extern crate hamcrest;

use balertest::support::{project, execs};
use hamcrest::assert_that;

#[test]
fn no_deps() {
    let p = project("foo")
        .file("Baler.toml", r#"
            [package]
            name = "foo"
            authors = []
            version = "0.0.1"
        "#)
        .file("src/main.rs", r#"
            mod a; fn main() {}
        "#)
        .file("src/a.rs", "");

    assert_that(p.baler_process("fetch"),
                execs().with_status(0).with_stdout(""));
}
