extern crate balertest;
extern crate hamcrest;

use std::fs::File;

use balertest::sleep_ms;
use balertest::support::{project, execs};
use hamcrest::assert_that;

#[test]
fn rerun_if_env_changes() {
    let p = project("foo")
        .file("Baler.toml", r#"
            [package]
            name = "foo"
            version = "0.5.0"
            authors = []
        "#)
        .file("src/main.rs", r#"
            fn main() {}
        "#)
        .file("build.rs", r#"
            fn main() {
                println!("baler:rerun-if-env-changed=FOO");
            }
        "#);
    p.build();

    assert_that(p.baler("build"),
                execs().with_status(0)
                       .with_stderr("\
[COMPILING] foo v0.5.0 ([..])
[FINISHED] [..]
"));
    assert_that(p.baler("build").env("FOO", "bar"),
                execs().with_status(0)
                       .with_stderr("\
[COMPILING] foo v0.5.0 ([..])
[FINISHED] [..]
"));
    assert_that(p.baler("build").env("FOO", "baz"),
                execs().with_status(0)
                       .with_stderr("\
[COMPILING] foo v0.5.0 ([..])
[FINISHED] [..]
"));
    assert_that(p.baler("build").env("FOO", "baz"),
                execs().with_status(0)
                       .with_stderr("\
[FINISHED] [..]
"));
    assert_that(p.baler("build"),
                execs().with_status(0)
                       .with_stderr("\
[COMPILING] foo v0.5.0 ([..])
[FINISHED] [..]
"));
}

#[test]
fn rerun_if_env_or_file_changes() {
    let p = project("foo")
        .file("Baler.toml", r#"
            [package]
            name = "foo"
            version = "0.5.0"
            authors = []
        "#)
        .file("src/main.rs", r#"
            fn main() {}
        "#)
        .file("build.rs", r#"
            fn main() {
                println!("baler:rerun-if-env-changed=FOO");
                println!("baler:rerun-if-changed=foo");
            }
        "#)
        .file("foo", "");
    p.build();

    assert_that(p.baler("build"),
                execs().with_status(0)
                       .with_stderr("\
[COMPILING] foo v0.5.0 ([..])
[FINISHED] [..]
"));
    assert_that(p.baler("build").env("FOO", "bar"),
                execs().with_status(0)
                       .with_stderr("\
[COMPILING] foo v0.5.0 ([..])
[FINISHED] [..]
"));
    assert_that(p.baler("build").env("FOO", "bar"),
                execs().with_status(0)
                       .with_stderr("\
[FINISHED] [..]
"));
    sleep_ms(1000);
    File::create(p.root().join("foo")).unwrap();
    assert_that(p.baler("build").env("FOO", "bar"),
                execs().with_status(0)
                       .with_stderr("\
[COMPILING] foo v0.5.0 ([..])
[FINISHED] [..]
"));
}
