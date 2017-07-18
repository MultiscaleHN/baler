extern crate baler;
extern crate balertest;
extern crate hamcrest;

use balertest::support::{project, execs};
use hamcrest::assert_that;

#[test]
fn simple() {
    let p = project("foo");
    p.build();

    assert_that(p.baler("version"),
                execs().with_status(0).with_stdout(&format!("{}\n",
                                                            baler::version())));

    assert_that(p.baler("--version"),
                execs().with_status(0).with_stdout(&format!("{}\n",
                                                            baler::version())));

}


#[test]
#[cfg_attr(target_os = "windows", ignore)]
fn version_works_without_rustc() {
    let p = project("foo");
    assert_that(p.baler_process("version").env("PATH", ""),
                execs().with_status(0));
}

#[test]
fn version_works_with_bad_config() {
    let p = project("foo")
        .file(".baler/config", "this is not toml");
    assert_that(p.baler_process("version"),
                execs().with_status(0));
}

#[test]
fn version_works_with_bad_target_dir() {
    let p = project("foo")
        .file(".baler/config", r#"
            [build]
            target-dir = 4
        "#);
    assert_that(p.baler_process("version"),
                execs().with_status(0));
}
