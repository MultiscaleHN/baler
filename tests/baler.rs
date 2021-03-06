extern crate baler;
extern crate balertest;
extern crate hamcrest;

use std::env;
use std::ffi::OsString;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::str;

use balertest::baler_process;
use balertest::support::paths::{self, CargoPathExt};
use balertest::support::{execs, project, ProjectBuilder, basic_bin_manifest};
use hamcrest::{assert_that, existing_file};

#[cfg_attr(windows,allow(dead_code))]
enum FakeKind<'a> {
    Executable,
    Symlink{target:&'a Path},
}

/// Add an empty file with executable flags (and platform-dependent suffix).
/// TODO: move this to `ProjectBuilder` if other cases using this emerge.
fn fake_file(proj: ProjectBuilder, dir: &Path, name: &str, kind: FakeKind) -> ProjectBuilder {
    let path = proj.root().join(dir).join(&format!("{}{}", name,
                                                   env::consts::EXE_SUFFIX));
    path.parent().unwrap().mkdir_p();
    match kind {
        FakeKind::Executable => {
            File::create(&path).unwrap();
            make_executable(&path);
        },
        FakeKind::Symlink{target} => {
            make_symlink(&path,target);
        }
    }
    return proj;

    #[cfg(unix)]
    fn make_executable(p: &Path) {
        use std::os::unix::prelude::*;

        let mut perms = fs::metadata(p).unwrap().permissions();
        let mode = perms.mode();
        perms.set_mode(mode | 0o111);
        fs::set_permissions(p, perms).unwrap();
    }
    #[cfg(windows)]
    fn make_executable(_: &Path) {}
    #[cfg(unix)]
    fn make_symlink(p: &Path, t: &Path) {
        ::std::os::unix::fs::symlink(t,p).expect("Failed to create symlink");
    }
    #[cfg(windows)]
    fn make_symlink(_: &Path, _: &Path) {
        panic!("Not supported")
    }
}

fn path() -> Vec<PathBuf> {
    env::split_paths(&env::var_os("PATH").unwrap_or(OsString::new())).collect()
}

#[test]
fn list_command_looks_at_path() {
    let proj = project("list-non-overlapping");
    let proj = fake_file(proj, Path::new("path-test"), "baler-1", FakeKind::Executable);
    let mut pr = baler_process();

    let mut path = path();
    path.push(proj.root().join("path-test"));
    let path = env::join_paths(path.iter()).unwrap();
    let output = pr.arg("-v").arg("--list")
                   .env("PATH", &path);
    let output = output.exec_with_output().unwrap();
    let output = str::from_utf8(&output.stdout).unwrap();
    assert!(output.contains("\n    1\n"), "missing 1: {}", output);
}

// windows and symlinks don't currently agree that well
#[cfg(unix)]
#[test]
fn list_command_resolves_symlinks() {
    use balertest::support::baler_exe;

    let proj = project("list-non-overlapping");
    let proj = fake_file(proj, Path::new("path-test"), "baler-2",
                         FakeKind::Symlink{target:&baler_exe()});
    let mut pr = baler_process();

    let mut path = path();
    path.push(proj.root().join("path-test"));
    let path = env::join_paths(path.iter()).unwrap();
    let output = pr.arg("-v").arg("--list")
                   .env("PATH", &path);
    let output = output.exec_with_output().unwrap();
    let output = str::from_utf8(&output.stdout).unwrap();
    assert!(output.contains("\n    2\n"), "missing 2: {}", output);
}

#[test]
fn find_closest_biuld_to_build() {
    let mut pr = baler_process();
    pr.arg("biuld");

    assert_that(pr,
                execs().with_status(101)
                       .with_stderr("[ERROR] no such subcommand: `biuld`

<tab>Did you mean `build`?

"));
}

// if a subcommand is more than 3 edit distance away, we don't make a suggestion
#[test]
fn find_closest_dont_correct_nonsense() {
    let mut pr = baler_process();
    pr.arg("there-is-no-way-that-there-is-a-command-close-to-this")
      .cwd(&paths::root());

    assert_that(pr,
                execs().with_status(101)
                       .with_stderr("[ERROR] no such subcommand: \
                        `there-is-no-way-that-there-is-a-command-close-to-this`
"));
}

#[test]
fn displays_subcommand_on_error() {
    let mut pr = baler_process();
    pr.arg("invalid-command");

    assert_that(pr,
                execs().with_status(101)
                       .with_stderr("[ERROR] no such subcommand: `invalid-command`
"));
}

#[test]
fn override_baler_home() {
    let root = paths::root();
    let my_home = root.join("my_home");
    fs::create_dir(&my_home).unwrap();
    File::create(&my_home.join("config")).unwrap().write_all(br#"
        [baler-new]
        name = "foo"
        email = "bar"
        git = false
    "#).unwrap();

    assert_that(baler_process()
                    .arg("new").arg("foo")
                    .env("USER", "foo")
                    .env("CARGO_HOME", &my_home),
                execs().with_status(0));

    let toml = paths::root().join("foo/Baler.toml");
    let mut contents = String::new();
    File::open(&toml).unwrap().read_to_string(&mut contents).unwrap();
    assert!(contents.contains(r#"authors = ["foo <bar>"]"#));
}

#[test]
fn baler_subcommand_env() {
    use balertest::support::baler_exe;

    let src = format!(r#"
        use std::env;

        fn main() {{
            println!("{{}}", env::var("{}").unwrap());
        }}
        "#, baler::CARGO_ENV);

    let p = project("baler-envtest")
        .file("Baler.toml", &basic_bin_manifest("baler-envtest"))
        .file("src/main.rs", &src);

    let target_dir = p.target_debug_dir();

    assert_that(p.baler_process("build"), execs().with_status(0));
    assert_that(&p.bin("baler-envtest"), existing_file());

    let mut pr = baler_process();
    let baler = baler_exe().canonicalize().unwrap();
    let mut path = path();
    path.push(target_dir);
    let path = env::join_paths(path.iter()).unwrap();

    assert_that(pr.arg("envtest").env("PATH", &path),
                execs().with_status(0).with_stdout(baler.to_str().unwrap()));
}

#[test]
fn baler_help() {
    assert_that(baler_process(),
                execs().with_status(0));
    assert_that(baler_process().arg("help"),
                execs().with_status(0));
    assert_that(baler_process().arg("-h"),
                execs().with_status(0));
    assert_that(baler_process().arg("help").arg("build"),
                execs().with_status(0));
    assert_that(baler_process().arg("build").arg("-h"),
                execs().with_status(0));
    assert_that(baler_process().arg("help").arg("-h"),
                execs().with_status(0));
    assert_that(baler_process().arg("help").arg("help"),
                execs().with_status(0));
}

#[test]
fn explain() {
    assert_that(baler_process().arg("--explain").arg("E0001"),
                execs().with_status(0));
}
