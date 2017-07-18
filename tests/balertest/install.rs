use std::fmt;
use std::path::{PathBuf, Path};

use hamcrest::{Matcher, MatchResult, existing_file};
use support::paths;

pub use self::InstalledExe as has_installed_exe;

pub fn baler_home() -> PathBuf {
    paths::home().join(".baler")
}

pub struct InstalledExe(pub &'static str);

fn exe(name: &str) -> String {
    if cfg!(windows) {format!("{}.exe", name)} else {name.to_string()}
}

impl<P: AsRef<Path>> Matcher<P> for InstalledExe {
    fn matches(&self, path: P) -> MatchResult {
        let path = path.as_ref().join("bin").join(exe(self.0));
        existing_file().matches(&path)
    }
}

impl fmt::Display for InstalledExe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "installed exe `{}`", self.0)
    }
}
