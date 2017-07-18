use std::env;

use baler;
use baler::util::{CliResult, Config};

#[derive(Deserialize)]
pub struct Options {}

pub const USAGE: &'static str = "
Show version information

Usage:
    baler version [options]

Options:
    -h, --help               Print this message
    -v, --verbose ...        Use verbose output (-vv very verbose/build.rs output)
    --color WHEN             Coloring: auto, always, never
";

pub fn execute(_: Options, _: &Config) -> CliResult {
    debug!("executing; cmd=baler-version; args={:?}", env::args().collect::<Vec<_>>());

    println!("{}", baler::version());

    Ok(())
}
