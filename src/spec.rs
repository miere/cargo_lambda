//! Reads the Rust LambdaSpec specification.

use crate::error::Result;

use toml;
use std::fs;
use std::path::PathBuf;
use serde_derive::Deserialize;

static DEFAULT_COMMAND: &str = "cargo build --release";

/// Represents a Cargo.toml file structure. It shall not be used outside of this
/// module, being here for the sake of simplicity as it allows "serde" to automatically
/// deserialize this file.
#[derive(Deserialize)]
struct CargoTomlSpec {

    package: CargoPackageSpec,

    #[serde(default)]
    bin: Vec<CargoBinSpec>,

    #[serde(default)]
    lambda: LambdaSpec
}

#[derive(Deserialize)]
struct CargoPackageSpec {
    name: String,
}

#[derive(Debug, Deserialize)]
struct CargoBinSpec {
    name: String,
}

/// Stores the LambdaSpec configuration and how it should behave once deployed.
#[derive(Deserialize)]
pub struct LambdaSpec {

    pub artifact_name: Option<String>,
    pub output_package_path: Option<String>,
    pub build_command: Option<String>,
    #[serde(default)]
    pub extra_artifacts: Vec<String>
}

impl Default for LambdaSpec {

    fn default() -> Self {
        LambdaSpec {
            artifact_name: None,
            output_package_path: None,
            build_command: Some(DEFAULT_COMMAND.to_string()),
            extra_artifacts: Default::default()
        }
    }
}

/// Reads the deployment spec from Cargo.toml file present on the current project.
pub fn read_project_cargo_toml() -> Result<LambdaSpec> {
    let project_cargo_toml = compute_cargo_toml_path()?;
    read_cargo_toml(&project_cargo_toml)
}

fn compute_cargo_toml_path() -> Result<String> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("Cargo.toml");
    Ok(path.to_str().unwrap().to_string())
}

/// Reads a Cargo.toml file and extracts all a vector of specs containing
/// everything needed to be handled by this plugin.
pub fn read_cargo_toml(file_name: &str) -> Result<LambdaSpec> {
    let content: String = fs::read_to_string(file_name)?;
    let cargo_toml: CargoTomlSpec = toml::from_str(&content)?;

    let specs = cargo_toml.bin.iter()
        .map( |bin| bin.name.clone())
        .collect::<Vec<String>>();

    let spec = LambdaSpec {
        artifact_name: Some(cargo_toml.package.name.clone()),
        output_package_path: Some(compute_output_package_path(&cargo_toml)),
        extra_artifacts: specs,
        ..cargo_toml.lambda
    };

    Ok(spec)
}

fn compute_output_package_path(cargo_toml: &CargoTomlSpec) -> String {
    match &cargo_toml.lambda.output_package_path {
        None => "target".to_owned(),
        Some(name) => name.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn should_read_minimalist_file() {
        let file = resource_file("Minimalist.toml");
        let lambda_spec = read_cargo_toml(&file).unwrap();

        assert_eq!("sample", lambda_spec.artifact_name.unwrap());
        assert_eq!("target", lambda_spec.output_package_path.unwrap());
        assert_eq!("cargo build --release", &lambda_spec.build_command.unwrap());
    }

    #[test]
    fn should_read_complex_file() {
        let file = resource_file("Complex.toml");
        let lambda_spec = read_cargo_toml(&file).unwrap();

        assert_eq!("hello-world", lambda_spec.artifact_name.unwrap());
        assert_eq!("hello-world.zip", lambda_spec.output_package_path.unwrap());
        assert_eq!("cargo build", &lambda_spec.build_command.unwrap());
    }

    fn resource_file(file_name: &str) -> String {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("test-resources");
        d.push(file_name);
        d.to_str().unwrap().to_string()
    }
}