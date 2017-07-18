use core::{Resolve, PackageSet, Workspace};
use ops;
use util::CargoResult;

/// Executes `baler fetch`.
pub fn fetch<'a>(ws: &Workspace<'a>) -> CargoResult<(Resolve, PackageSet<'a>)> {
    let (packages, resolve) = ops::resolve_ws(ws)?;
    for id in resolve.iter() {
        packages.get(id)?;
    }
    Ok((resolve, packages))
}
