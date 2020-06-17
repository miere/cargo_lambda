use std::process::Output;
use crate::error::Result;
use crate::error::KnownFailures;
use std::process::Command as OsCommand;
use std::env;
use std::path::PathBuf;

static DOCKER_IMAGE: &str = "ekidd/rust-musl-builder";
static CMD_FIX_PERMISSION: &str = "sudo chown -R rust:rust /home/rust/src/target";
pub static MUSL_OUTPUT_DIR: &str = "target/x86_64-unknown-linux-musl/release";

pub struct DockerCommand {
    current_dir: String,
    command: String,
    rust_version: String
}

impl DockerCommand {

    pub fn new(command: String, rust_version: Option<String>) -> DockerCommand {
        let current_dir = env::current_dir().unwrap().to_str().unwrap().to_owned();
        let rust_version = rust_version.unwrap_or_else(|| "stable".to_owned());
        DockerCommand { current_dir, rust_version, command }
    }
}

/// Extracts the output folder where the binary will be placed once the
/// build process is successfully finished.
/// 
/// It worth notice [cmd] is not being used. This parameter is here for
/// further usage, where we can have multiple Docker images for different
/// purposes. On such cases, we might need to enhance DockerCommand struct
/// allowing this function to retrieve the output path.
pub fn output_path(_cmd: &DockerCommand) -> Result<String> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push(MUSL_OUTPUT_DIR);
    Ok(path.to_str().unwrap().to_string())
}

/// Runs a DockerCommand. It was designed to run any linux command on a
/// Rust-compatible Docker image.
pub fn run(cmd: &DockerCommand) -> Result<()> {
    let output = spin_up_process(cmd)?;

    consume_log(&output.stdout);
    consume_log(&output.stderr);

    if !output.status.success() {
        return Err(KnownFailures::FailedToCompile)
    }

    Ok(())
}

fn spin_up_process<'a>(cmd: &DockerCommand) -> Result<Output> {
    let home = env::var("HOME").unwrap();
    let cargo_git_vol = format!("cargo-git:{}/.cargo/git", &home);
    let cargo_registry_vol = format!("cargo-registry:{}/.cargo/registry", &home);
    let source_vol = format!("{}:/home/rust/src", &cmd.current_dir);
    let target_vol = format!("{}/target:/home/rust/src/target", &cmd.current_dir);
    let docker_image = format!("{}:{}", DOCKER_IMAGE, &cmd.rust_version);
    let command = format!("{} && {}", CMD_FIX_PERMISSION, &cmd.command);

    let output = OsCommand::new("docker")
        .arg("run").arg("--rm").arg("-t")
        .arg("-v").arg(cargo_git_vol)
        .arg("-v").arg(cargo_registry_vol)
        .arg("-v").arg(source_vol)
        .arg("-v").arg(target_vol)
        .arg(docker_image)
        .arg("sh").arg("-c").arg(command).output()?;

    Ok(output)
}

fn consume_log(output: &Vec<u8>) {
    String::from_utf8_lossy(output)
        .lines()
        .for_each(|line| println!("{}", line));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_compile_this_project_via_docker() -> Result<()> {
        let cmd = DockerCommand::new("cargo help".to_owned(), None);
        run(&cmd)
    }

}
