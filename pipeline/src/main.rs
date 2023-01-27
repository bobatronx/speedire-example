use carrier::tools::config::{self, Tool};
use carrier::tools::poetry_setup::Poetry;

fn main() {
    config::initialize().unwrap();
    let poetry = Poetry::default();
    poetry.configure().unwrap();
    poetry.execute_with_args(&["update"]).unwrap();
    poetry.execute("build").unwrap();
    config::cleanup().unwrap();
}
