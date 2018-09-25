mod nuke;

fn main() -> std::io::Result<()> {
    nuke::nuke("./node_modules_nuke/node_modules")
}
