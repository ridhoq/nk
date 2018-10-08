extern crate dunce;
extern crate nuke;
#[macro_use]
extern crate structopt;

use std::io;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "nuke", author = "")]
struct Opt {
    /// Force delete without confirmation
    #[structopt(short = "f", long = "force")]
    force: bool,
    /// Path to delete
    #[structopt(name = "path to delete", parse(from_os_str))]
    path: PathBuf,
}

fn main() -> std::io::Result<()> {
    let opt = Opt::from_args();

    if opt.path.exists() && !opt.force {
        println!(
            "Press Y to nuke: {}",
            dunce::canonicalize(&opt.path).unwrap().display()
        );
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if input.trim().to_uppercase() != "Y" {
            println!("nuke aborted");
            return Ok(());
        }
    }

    nuke::nuke(&opt.path)
}
