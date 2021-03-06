extern crate balertest;
extern crate hamcrest;

use balertest::support::{project, execs, main_file, basic_bin_manifest};
use hamcrest::{assert_that};

static MANIFEST_OUTPUT: &'static str = r#"
{
    "name":"foo",
    "version":"0.5.0",
    "id":"foo[..]0.5.0[..](path+file://[..]/foo)",
    "license": null,
    "license_file": null,
    "description": null,
    "source":null,
    "dependencies":[],
    "targets":[{
        "kind":["bin"],
        "crate_types":["bin"],
        "name":"foo",
        "src_path":"[..][/]foo[/]src[/]foo.rs"
    }],
    "features":{},
    "manifest_path":"[..]Baler.toml"
}"#;

#[test]
fn baler_read_manifest_path_to_baler_toml_relative() {
    let p = project("foo")
        .file("Baler.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]));

    assert_that(p.baler_process("read-manifest")
                 .arg("--manifest-path").arg("foo/Baler.toml")
                 .cwd(p.root().parent().unwrap()),
                execs().with_status(0)
                       .with_json(MANIFEST_OUTPUT));
}

#[test]
fn baler_read_manifest_path_to_baler_toml_absolute() {
    let p = project("foo")
        .file("Baler.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]));

    assert_that(p.baler_process("read-manifest")
                 .arg("--manifest-path").arg(p.root().join("Baler.toml"))
                 .cwd(p.root().parent().unwrap()),
                execs().with_status(0)
                       .with_json(MANIFEST_OUTPUT));
}

#[test]
fn baler_read_manifest_path_to_baler_toml_parent_relative() {
    let p = project("foo")
        .file("Baler.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]));

    assert_that(p.baler_process("read-manifest")
                 .arg("--manifest-path").arg("foo")
                 .cwd(p.root().parent().unwrap()),
                execs().with_status(101)
                       .with_stderr("[ERROR] the manifest-path must be \
                                             a path to a Baler.toml file"));
}

#[test]
fn baler_read_manifest_path_to_baler_toml_parent_absolute() {
    let p = project("foo")
        .file("Baler.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]));

    assert_that(p.baler_process("read-manifest")
                 .arg("--manifest-path").arg(p.root())
                 .cwd(p.root().parent().unwrap()),
                execs().with_status(101)
                       .with_stderr("[ERROR] the manifest-path must be \
                                             a path to a Baler.toml file"));
}

#[test]
fn baler_read_manifest_cwd() {
    let p = project("foo")
        .file("Baler.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]));

    assert_that(p.baler_process("read-manifest")
                 .cwd(p.root()),
                execs().with_status(0)
                       .with_json(MANIFEST_OUTPUT));
}
