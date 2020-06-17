mod spec;
mod error;
mod docker;
mod lambda;
mod log;

use clap::arg_enum;
use structopt::StructOpt;
use docker::DockerCommand;
use error::Result;

arg_enum! {
    #[derive(Debug, StructOpt)]
    pub enum Task {
        Package,
        Build,
        Lambda, // required to avoid conflicts with the cargo command line syntax
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "cargo_lambda")]
struct Opt {

    /// The Rust version used
    #[structopt(short, long, default_value = "stable")]
    rust_version: String,

    /// Commands to process
    #[structopt(required = true, possible_values = &Task::variants(), min_values = 2, case_insensitive = true)]
    commands: Vec<Task>,
}

fn main() {
    let opt = Opt::from_args();

    for cmd in opt.commands.iter() {
        let result = match cmd {
            Task::Build => build(&opt),
            Task::Package => package(&opt),
            _ => Ok(())
        };

        if let Err(m) = result {
            log::warn(format!("Failed to execute task '{}'", cmd));
            log::error(format!("{}", m));
            break;
        }
    }
}

fn build(opt: &Opt) -> Result<()> {
    log::info("Compiling as Lambda compatible binary.".to_owned());
    log::info("Using Docker to build the Lambda image. It may take a while to print the output messages.".to_owned());

    let lambda_spec = read_lambda_spec(opt)?;
    let cmd = create_docker_command(opt, &lambda_spec);
    docker::run(&cmd)
}

fn package(opt: &Opt) -> Result<()> {
    log::info("Generating Lambda package...".to_owned());

    let lambda_spec = read_lambda_spec(opt)?;
    let cmd = create_docker_command(opt, &lambda_spec);
    let docker_output_path = docker::output_path(&cmd)?;
    let output_package_path = lambda_spec.output_package_path.unwrap();

    create_package( &output_package_path, &docker_output_path, &lambda_spec.artifact_name.unwrap() )?;

    for artifact_name in lambda_spec.extra_artifacts.iter() {
        create_package( &output_package_path, &docker_output_path, artifact_name )?
    }

    Ok(())
}

fn create_package(
    output_package_path: &str,
    docker_output_path: &str,
    artifact_name: &str) -> Result<()>
{
    let bootstrap_file_path = format!("{}/{}", docker_output_path, artifact_name);
    let artifact_package_path = format!("{}/{}-lambda.zip", output_package_path, artifact_name);

    lambda::package_binary( &artifact_package_path, &bootstrap_file_path )?;

    log::info(format!("Package generated: {}", &artifact_package_path));
    Ok(())
}

fn read_lambda_spec(_opt: &Opt) -> Result<spec::LambdaSpec> {
    spec::read_project_cargo_toml()
}

fn create_docker_command(opt: &Opt, lambda_spec: &spec::LambdaSpec) -> DockerCommand {
    DockerCommand::new(
        lambda_spec.build_command.clone().unwrap(),
        Some(opt.rust_version.clone())
    )
}