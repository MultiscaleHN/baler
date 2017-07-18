use baler::util::{CliResult, CliError, Config};

#[derive(Deserialize)]
pub struct Options;

pub const USAGE: &'static str = "
Get some help with a baler command.

Usage:
    baler help <command>
    baler help -h | --help

Options:
    -h, --help          Print this message
";

pub fn execute(_: Options, _: &Config) -> CliResult {
    // This is a dummy command just so that `baler help help` works.
    // The actual delegation of help flag to subcommands is handled by the
    // baler command.
    Err(CliError::new("help command should not be executed directly".into(), 101))
}
