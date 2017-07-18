extern crate balertest;
extern crate hamcrest;
use balertest::support::{project, execs, basic_bin_manifest};
use hamcrest::{assert_that};

#[test]
fn alias_incorrect_config_type() {
    let p = project("foo")
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/main.rs", r#"
            fn main() {
        }"#)
        .file(".baler/config",r#"
            [alias]
            b-baler-test = 5
        "#);

    assert_that(p.baler_process("b-baler-test").arg("-v"),
                execs().with_status(101).
                with_stderr_contains("[ERROR] invalid configuration \
for key `alias.b-baler-test`
expected a list, but found a integer for [..]"));
}


#[test]
fn alias_default_config_overrides_config() {
    let p = project("foo")
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/main.rs", r#"
            fn main() {
        }"#)
        .file(".baler/config",r#"
            [alias]
            b = "not_build"
        "#);

    assert_that(p.baler_process("b").arg("-v"),
                execs().with_status(0).
                with_stderr_contains("[COMPILING] foo v0.5.0 [..]"));
}

#[test]
fn alias_config() {
    let p = project("foo")
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/main.rs", r#"
            fn main() {
        }"#)
        .file(".baler/config",r#"
            [alias]
            b-baler-test = "build"
        "#);

    assert_that(p.baler_process("b-baler-test").arg("-v"),
                execs().with_status(0).
                with_stderr_contains("[COMPILING] foo v0.5.0 [..]
[RUNNING] `rustc --crate-name foo [..]"));
}

#[test]
fn alias_list_test() {
    let p = project("foo")
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/main.rs", r#"
            fn main() {
         }"#)
        .file(".baler/config",r#"
            [alias]
            b-baler-test = ["build", "--release"]
         "#);

    assert_that(p.baler_process("b-baler-test").arg("-v"),
                execs().with_status(0).
                with_stderr_contains("[COMPILING] foo v0.5.0 [..]").
                with_stderr_contains("[RUNNING] `rustc --crate-name [..]")
                );
}

#[test]
fn alias_with_flags_config() {
    let p = project("foo")
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/main.rs", r#"
            fn main() {
         }"#)
        .file(".baler/config",r#"
            [alias]
            b-baler-test = "build --release"
         "#);

    assert_that(p.baler_process("b-baler-test").arg("-v"),
                execs().with_status(0).
                with_stderr_contains("[COMPILING] foo v0.5.0 [..]").
                with_stderr_contains("[RUNNING] `rustc --crate-name foo [..]")
                );
}

#[test]
fn cant_shadow_builtin() {
    let p = project("foo")
        .file("Cargo.toml", &basic_bin_manifest("foo"))
        .file("src/main.rs", r#"
            fn main() {
         }"#)
        .file(".baler/config",r#"
            [alias]
            build = "fetch"
         "#);

    assert_that(p.baler_process("build"),
                execs().with_status(0)
                       .with_stderr("\
[COMPILING] foo v0.5.0 ([..])
[FINISHED] dev [unoptimized + debuginfo] target(s) in [..]
"));
}
