use speedire::toolfs;
use speedire::poetry_setup::Poetry;
use speedire::kubectl_setup::Kubectl;
use speedire::toolfs::{BuilderTool, DeployerTool};

fn main() {
    toolfs::initialize()
    .expect("unable to initialize");
    
    Poetry::default()
    .build()
    .expect("unable to execute poetry build");
    
    Kubectl::default()
    .namespace("speedire")
    .apply("k8s/deployment.yaml")
    .deploy()
    .expect("unable to execute kubectl");

    toolfs::cleanup()
    .expect("unable to cleanup");
}
