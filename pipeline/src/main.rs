use speedire::toolfs;
use speedire::poetry_setup::Poetry;
use speedire::kubectl_setup::Kubectl;
use speedire::toolfs::{Tool, BuilderTool, DeployerTool};

fn main() {
    toolfs::initialize()
    .expect("unable to initialize");
    
    let poetry = Poetry::default();
    poetry.configure()
    .and(poetry.build())
    .expect("unable to execute poetry build");
    
    let kubectl = Kubectl::default();
    kubectl.configure()
    .and(kubectl.set_namespace("speedire"))
    .and(kubectl.deploy(&["apply", "-f", "k8s/deployment.yaml"]))
    .expect("unable to execute kubectl");

    toolfs::cleanup()
    .expect("unable to cleanup");
}
