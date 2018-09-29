extern crate nuke;

fn main() -> std::io::Result<()> {
    nuke::nuke("./node_modules")
}
