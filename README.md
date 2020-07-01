# cargo_lambda
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fmiere%2Fcargo_lambda.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fmiere%2Fcargo_lambda?ref=badge_shield)

A Cargo plugin to generate and package musl-based binaries that can be uploaded as AWS Lambda functions.

## Installing
This project is not available via `cargo` yet, so you can build from the source code and manually install it on your PC.

```shell
$ git clone git@github.com:miere/cargo_lambda.git
$ cd cargo_lambda
$ cargo build --release
$ cp target/release/cargo_lambda $CARGO_HOME/bin
```

## Configuring your project
No changes on `Cargo.toml` have to be done to use `cargo_lambda` with the default configuration. In case the default values doesn't suit your needs, you can use the sample `[lambda]` section below as inspiration.

```toml
# ...
[lambda]
# Lambda packages will be placed here
output_package_path = "../my-custom-output-path/"
```

## Basic Usage
The basic syntax is `cargo lambda [OPTS]` where OPTS stands for:
- **build** - Spin up a Docker image to build the software and generate an AWS Lambda compatible binary
- **package** - Generates a package for each binary configured on your `Cargo.toml`, including the root project.

Sample command that builds and generates an AWS Lambda zip file.
```shell
$ cargo lambda build package
Compiling as Lambda compatible binary.
Using Docker to build the Lambda image. It may take a while to print the output messages.
    Updating crates.io index
 Downloading crates ...
  Downloaded tokio v0.2.21
  <<trimmed for simplicity>>
   Compiling cargo-serverless v0.1.0 (/home/rust/src)
    Finished release [optimized] target(s) in 58.41s
Generating Lambda package...
Package generated: target/ice-lambda.zip
Package generated: target/cream-lambda.zip
```

## Requirements
As this plugin uses Docker to build the software. Please make sure Docker daemon is running and your user doesn't require `sudo` to execute command line instructions.

## Reporting Bugs/Feature Requests
We welcome you to use the GitHub issue tracker to report bugs or suggest features.

When filing an issue, please check existing open, or recently closed, issues to make sure somebody else hasn't already
reported the issue. Please try to include as much information as you can. Details like these are incredibly useful:

* A reproducible test case or series of steps
* The version of our code being used
* Any modifications you've made relevant to the bug
* Anything unusual about your environment or deployment

## Contributing via Pull Requests
Contributions via pull requests are much appreciated. Before sending us a pull request, please ensure that:

1. You are working against the latest source on the *master* branch.
2. You check existing open, and recently merged, pull requests to make sure someone else hasn't addressed the problem already.
3. You open an issue to discuss any significant work - we would hate for your time to be wasted.

To send us a pull request, please:

1. Fork the repository.
2. Modify the source; please focus on the specific change you are contributing. If you also reformat all the code, it will be hard for us to focus on your change.
3. Ensure local tests pass.
4. Commit to your fork using clear commit messages.
5. Send us a pull request, answering any default questions in the pull request interface.
6. Pay attention to any automated CI failures reported in the pull request, and stay involved in the conversation.

GitHub provides additional document on [forking a repository](https://help.github.com/articles/fork-a-repo/) and
[creating a pull request](https://help.github.com/articles/creating-a-pull-request/).

## Finding contributions to work on
Looking at the existing issues is a great way to find something to contribute on. As our projects, by default, use the default GitHub issue labels ((enhancement/bug/duplicate/help wanted/invalid/question/wontfix), looking at any 'help wanted' issues is a great place to start.

## License
This project is released under the Apache License 2 terms.

[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fmiere%2Fcargo_lambda.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fmiere%2Fcargo_lambda?ref=badge_large)