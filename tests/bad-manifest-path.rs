extern crate hamcrest;
extern crate balertest;

use balertest::support::{project, execs, main_file, basic_bin_manifest};
use hamcrest::{assert_that};

fn assert_not_a_baler_toml(command: &str, manifest_path_argument: &str) {
    let p = project("foo")
        .file("Baler.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]));

    assert_that(p.baler_process(command)
                 .arg("--manifest-path").arg(manifest_path_argument)
                 .cwd(p.root().parent().unwrap()),
                execs().with_status(101)
                       .with_stderr("[ERROR] the manifest-path must be a path \
                                             to a Baler.toml file"));
}


fn assert_baler_toml_doesnt_exist(command: &str, manifest_path_argument: &str) {
    let p = project("foo");
    let expected_path = manifest_path_argument
        .split('/').collect::<Vec<_>>().join("[..]");

    assert_that(p.baler_process(command)
                 .arg("--manifest-path").arg(manifest_path_argument)
                 .cwd(p.root().parent().unwrap()),
                execs().with_status(101)
                       .with_stderr(
                           format!("[ERROR] manifest path `{}` does not exist",
                                   expected_path)
                       ));
}

#[test]
fn bench_dir_containing_baler_toml() {
    assert_not_a_baler_toml("bench", "foo");
}

#[test]
fn bench_dir_plus_file() {
    assert_not_a_baler_toml("bench", "foo/bar");
}

#[test]
fn bench_dir_plus_path() {
    assert_not_a_baler_toml("bench", "foo/bar/baz");
}

#[test]
fn bench_dir_to_nonexistent_baler_toml() {
    assert_baler_toml_doesnt_exist("bench", "foo/bar/baz/Baler.toml");
}

#[test]
fn build_dir_containing_baler_toml() {
    assert_not_a_baler_toml("build", "foo");
}

#[test]
fn build_dir_plus_file() {
    assert_not_a_baler_toml("bench", "foo/bar");
}

#[test]
fn build_dir_plus_path() {
    assert_not_a_baler_toml("bench", "foo/bar/baz");
}

#[test]
fn build_dir_to_nonexistent_baler_toml() {
    assert_baler_toml_doesnt_exist("build", "foo/bar/baz/Baler.toml");
}

#[test]
fn clean_dir_containing_baler_toml() {
    assert_not_a_baler_toml("clean", "foo");
}

#[test]
fn clean_dir_plus_file() {
    assert_not_a_baler_toml("clean", "foo/bar");
}

#[test]
fn clean_dir_plus_path() {
    assert_not_a_baler_toml("clean", "foo/bar/baz");
}

#[test]
fn clean_dir_to_nonexistent_baler_toml() {
    assert_baler_toml_doesnt_exist("clean", "foo/bar/baz/Baler.toml");
}

#[test]
fn doc_dir_containing_baler_toml() {
    assert_not_a_baler_toml("doc", "foo");
}

#[test]
fn doc_dir_plus_file() {
    assert_not_a_baler_toml("doc", "foo/bar");
}

#[test]
fn doc_dir_plus_path() {
    assert_not_a_baler_toml("doc", "foo/bar/baz");
}

#[test]
fn doc_dir_to_nonexistent_baler_toml() {
    assert_baler_toml_doesnt_exist("doc", "foo/bar/baz/Baler.toml");
}

#[test]
fn fetch_dir_containing_baler_toml() {
    assert_not_a_baler_toml("fetch", "foo");
}

#[test]
fn fetch_dir_plus_file() {
    assert_not_a_baler_toml("fetch", "foo/bar");
}

#[test]
fn fetch_dir_plus_path() {
    assert_not_a_baler_toml("fetch", "foo/bar/baz");
}

#[test]
fn fetch_dir_to_nonexistent_baler_toml() {
    assert_baler_toml_doesnt_exist("fetch", "foo/bar/baz/Baler.toml");
}

#[test]
fn generate_lockfile_dir_containing_baler_toml() {
    assert_not_a_baler_toml("generate-lockfile", "foo");
}

#[test]
fn generate_lockfile_dir_plus_file() {
    assert_not_a_baler_toml("generate-lockfile", "foo/bar");
}

#[test]
fn generate_lockfile_dir_plus_path() {
    assert_not_a_baler_toml("generate-lockfile", "foo/bar/baz");
}

#[test]
fn generate_lockfile_dir_to_nonexistent_baler_toml() {
    assert_baler_toml_doesnt_exist("generate-lockfile", "foo/bar/baz/Baler.toml");
}

#[test]
fn package_dir_containing_baler_toml() {
    assert_not_a_baler_toml("package", "foo");
}

#[test]
fn package_dir_plus_file() {
    assert_not_a_baler_toml("package", "foo/bar");
}

#[test]
fn package_dir_plus_path() {
    assert_not_a_baler_toml("package", "foo/bar/baz");
}

#[test]
fn package_dir_to_nonexistent_baler_toml() {
    assert_baler_toml_doesnt_exist("package", "foo/bar/baz/Baler.toml");
}

#[test]
fn pkgid_dir_containing_baler_toml() {
    assert_not_a_baler_toml("pkgid", "foo");
}

#[test]
fn pkgid_dir_plus_file() {
    assert_not_a_baler_toml("pkgid", "foo/bar");
}

#[test]
fn pkgid_dir_plus_path() {
    assert_not_a_baler_toml("pkgid", "foo/bar/baz");
}

#[test]
fn pkgid_dir_to_nonexistent_baler_toml() {
    assert_baler_toml_doesnt_exist("pkgid", "foo/bar/baz/Baler.toml");
}

#[test]
fn publish_dir_containing_baler_toml() {
    assert_not_a_baler_toml("publish", "foo");
}

#[test]
fn publish_dir_plus_file() {
    assert_not_a_baler_toml("publish", "foo/bar");
}

#[test]
fn publish_dir_plus_path() {
    assert_not_a_baler_toml("publish", "foo/bar/baz");
}

#[test]
fn publish_dir_to_nonexistent_baler_toml() {
    assert_baler_toml_doesnt_exist("publish", "foo/bar/baz/Baler.toml");
}

#[test]
fn read_manifest_dir_containing_baler_toml() {
    assert_not_a_baler_toml("read-manifest", "foo");
}

#[test]
fn read_manifest_dir_plus_file() {
    assert_not_a_baler_toml("read-manifest", "foo/bar");
}

#[test]
fn read_manifest_dir_plus_path() {
    assert_not_a_baler_toml("read-manifest", "foo/bar/baz");
}

#[test]
fn read_manifest_dir_to_nonexistent_baler_toml() {
    assert_baler_toml_doesnt_exist("read-manifest", "foo/bar/baz/Baler.toml");
}

#[test]
fn run_dir_containing_baler_toml() {
    assert_not_a_baler_toml("run", "foo");
}

#[test]
fn run_dir_plus_file() {
    assert_not_a_baler_toml("run", "foo/bar");
}

#[test]
fn run_dir_plus_path() {
    assert_not_a_baler_toml("run", "foo/bar/baz");
}

#[test]
fn run_dir_to_nonexistent_baler_toml() {
    assert_baler_toml_doesnt_exist("run", "foo/bar/baz/Baler.toml");
}

#[test]
fn rustc_dir_containing_baler_toml() {
    assert_not_a_baler_toml("rustc", "foo");
}

#[test]
fn rustc_dir_plus_file() {
    assert_not_a_baler_toml("rustc", "foo/bar");
}

#[test]
fn rustc_dir_plus_path() {
    assert_not_a_baler_toml("rustc", "foo/bar/baz");
}

#[test]
fn rustc_dir_to_nonexistent_baler_toml() {
    assert_baler_toml_doesnt_exist("rustc", "foo/bar/baz/Baler.toml");
}

#[test]
fn test_dir_containing_baler_toml() {
    assert_not_a_baler_toml("test", "foo");
}

#[test]
fn test_dir_plus_file() {
    assert_not_a_baler_toml("test", "foo/bar");
}

#[test]
fn test_dir_plus_path() {
    assert_not_a_baler_toml("test", "foo/bar/baz");
}

#[test]
fn test_dir_to_nonexistent_baler_toml() {
    assert_baler_toml_doesnt_exist("test", "foo/bar/baz/Baler.toml");
}

#[test]
fn update_dir_containing_baler_toml() {
    assert_not_a_baler_toml("update", "foo");
}

#[test]
fn update_dir_plus_file() {
    assert_not_a_baler_toml("update", "foo/bar");
}

#[test]
fn update_dir_plus_path() {
    assert_not_a_baler_toml("update", "foo/bar/baz");
}

#[test]
fn update_dir_to_nonexistent_baler_toml() {
    assert_baler_toml_doesnt_exist("update", "foo/bar/baz/Baler.toml");
}

#[test]
fn verify_project_dir_containing_baler_toml() {
    let p = project("foo")
        .file("Baler.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]));

    assert_that(p.baler_process("verify-project")
                 .arg("--manifest-path").arg("foo")
                 .cwd(p.root().parent().unwrap()),
                execs().with_status(1)
                       .with_stdout("\
{\"invalid\":\"the manifest-path must be a path to a Baler.toml file\"}\
                        "));
}

#[test]
fn verify_project_dir_plus_file() {
    let p = project("foo")
        .file("Baler.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]));

    assert_that(p.baler_process("verify-project")
                 .arg("--manifest-path").arg("foo/bar")
                 .cwd(p.root().parent().unwrap()),
                execs().with_status(1)
                       .with_stdout("\
{\"invalid\":\"the manifest-path must be a path to a Baler.toml file\"}\
                        "));
}

#[test]
fn verify_project_dir_plus_path() {
    let p = project("foo")
        .file("Baler.toml", &basic_bin_manifest("foo"))
        .file("src/foo.rs", &main_file(r#""i am foo""#, &[]));

    assert_that(p.baler_process("verify-project")
                 .arg("--manifest-path").arg("foo/bar/baz")
                 .cwd(p.root().parent().unwrap()),
                execs().with_status(1)
                       .with_stdout("\
{\"invalid\":\"the manifest-path must be a path to a Baler.toml file\"}\
                        "));
}

#[test]
fn verify_project_dir_to_nonexistent_baler_toml() {
    let p = project("foo");
    assert_that(p.baler_process("verify-project")
                 .arg("--manifest-path").arg("foo/bar/baz/Baler.toml")
                 .cwd(p.root().parent().unwrap()),
                execs().with_status(1)
                       .with_stdout("\
{\"invalid\":\"manifest path `foo[..]bar[..]baz[..]Baler.toml` does not exist\"}\
                        "));
}
