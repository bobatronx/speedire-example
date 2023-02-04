use speedire::toolfs;
use speedire::poetry_setup::Poetry;
use speedire::kubectl_setup::Kubectl;
use speedire::toolfs::Tool;

fn main() {
    toolfs::initialize().unwrap();
    
    let poetry = Poetry::default();
    poetry.configure().unwrap();
    poetry.execute_with_args(&["update"]).unwrap();
    poetry.execute("build").unwrap();
    poetry.execute_with_args(&["run", "--", "pytest"]).unwrap();
    
    let kubectl = Kubectl::default();
    kubectl.configure().unwrap();
    kubectl.set_namespace("speedire").unwrap();
    kubectl.execute_with_args(&["apply", "-f", "../k8s/deployment.yaml"]).unwrap();

    toolfs::cleanup().unwrap();
}
