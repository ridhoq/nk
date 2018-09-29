extern crate nuke;

use std::path::PathBuf;

fn main() -> std::io::Result<()> {
    nuke::nuke(&PathBuf::from("./node_modules_rmdir/node_modules"))
}
