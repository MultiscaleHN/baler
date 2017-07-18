use std::env;

use baler::core::Workspace;
use baler::ops;
use baler::util::{CliResult, Config};
use baler::util::important_paths::{find_root_manifest_for_wd};

#[derive(Deserialize)]
pub struct Options {
    flag_package: Vec<String>,
    flag_target: Option<String>,
    flag_manifest_path: Option<String>,
    flag_verbose: u32,
    flag_quiet: Option<bool>,
    flag_color: Option<String>,
    flag_release: bool,
    flag_frozen: bool,
    flag_locked: bool,
}

pub const USAGE: &'static str = "
Remove artifacts that baler has generated in the past

Usage:
    baler clean [options]

Options:
    -h, --help                   Print this message
    -p SPEC, --package SPEC ...  Package to clean artifacts for
    --manifest-path PATH         Path to the manifest to the package to clean
    --target TRIPLE              Target triple to clean output for (default all)
    --release                    Whether or not to clean release artifacts
    -v, --verbose ...            Use verbose output (-vv very verbose/build.rs output)
    -q, --quiet                  No output printed to stdout
    --color WHEN                 Coloring: auto, always, never
    --frozen                     Require Baler.lock and cache are up to date
    --locked                     Require Baler.lock is up to date

If the --package argument is given, then SPEC is a package id specification
which indicates which package's artifacts should be cleaned out. If it is not
given, then all packages' artifacts are removed. For more information on SPEC
and its format, see the `baler help pkgid` command.
";

pub fn execute(options: Options, config: &Config) -> CliResult {
    debug!("executing; cmd=baler-clean; args={:?}", env::args().collect::<Vec<_>>());
    config.configure(options.flag_verbose,
                     options.flag_quiet,
                     &options.flag_color,
                     options.flag_frozen,
                     options.flag_locked)?;

    let root = find_root_manifest_for_wd(options.flag_manifest_path, config.cwd())?;
    let opts = ops::CleanOptions {
        config: config,
        spec: &options.flag_package,
        target: options.flag_target.as_ref().map(|s| &s[..]),
        release: options.flag_release,
    };
    let ws = Workspace::new(&root, config)?;
    ops::clean(&ws, &opts)?;
    Ok(())
}
