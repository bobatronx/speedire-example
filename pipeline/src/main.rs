use speedire::pipelines::Speedire;
use speedire::pipelines::PipelineBuilder;
use speedire::commands::poetry::PoetryCommandBuilder;
use speedire::commands::kubectl::KubectlCommandBuilder;
use speedire::pipelines::PipelineDeployer;


fn main() {
    
    Speedire::new()
    .builder()
    .step(Box::new(
        PoetryCommandBuilder::new()
        .compile()
    ))
    .compile()
    .build()
    .expect("unable to run poetry build");

    Speedire::new()
    .deployer()
    .step(Box::new(
        KubectlCommandBuilder::new()
        .namespace("speedire")
        .apply("k8s/deployment.yaml")
        .compile()
    ))
    .compile()
    .deploy()
    .expect("unable to run kubectl apply");

    Speedire::destroy();
}
