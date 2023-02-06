use speedire::toolfs;
use speedire::poetry_setup::{PoetryCommandBuilder};
use speedire::pipelines::{BuildPipelineBuilder, DeployPipelineBuilder};
use speedire::kubectl_setup::{KubectlCommandBuilder};
use speedire::pipelines::{PipelineBuilder, PipelineDeployer};

fn main() {
    toolfs::initialize()
    .expect("unable to initialize");
    
    // Create build command
    let poetry_command = PoetryCommandBuilder::new()
    .compile();

    // Run Build
    BuildPipelineBuilder::new()
    .step(Box::new(poetry_command))
    .compile()
    .build()
    .expect("build failed");

    // Create deploy command
    let kubectl_command = KubectlCommandBuilder::new()
    .namespace("speedire")
    .apply("k8s/deployment.yaml")
    .compile();

    // Run Deploy
    DeployPipelineBuilder::new()
    .step(Box::new(kubectl_command))
    .compile()
    .deploy()
    .expect("build failed");
    
    toolfs::cleanup()
    .expect("unable to cleanup");
}
