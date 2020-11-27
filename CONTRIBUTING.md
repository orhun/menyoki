# Contributing <a href="https://github.com/orhun/menyoki"><img src="https://user-images.githubusercontent.com/24392180/99184076-96c10b00-2751-11eb-99ea-ad962144df76.png" width="25"></a>

Thank you for considering contributing to [menyoki](https://github.com/orhun/menyoki/)!

When contributing, please first discuss the change you wish to make via [issue](https://github.com/orhun/menyoki/issues),
[email](mailto:orhunparmaksiz@gmail.com), or any other method with the owners of this repository before making a change.

Note that we have a [Code of Conduct](https://github.com/orhun/menyoki/blob/master/.github/CODE_OF_CONDUCT.md), please follow it in all your interactions with the project.

## Setup

1. Fork this repository and create your branch from `master`.

2. Clone your forked repository.

```sh
git clone https://github.com/{username}/menyoki && cd menyoki
```

3. Make sure that you have [Rust](https://www.rust-lang.org/) `1.49.0-nightly` or later installed and build the project.
   
```sh
cargo build
```

4. See if the project is built without errors. If not, check if the system dependencies are installed and then go back to step 3.

5. Start committing your changes. You can use `--release` flag in the development phase for avoiding issues related to performance.

6. Add your tests (if you haven't already) or update the existing tests according to the changes. And check if the tests are passed.

```sh
cargo test
# Include the window system tests
cargo test --features test-ws
```

7. Make sure [rustfmt](https://github.com/rust-lang/rustfmt) and [clippy](https://github.com/rust-lang/rust-clippy) don't complain.

```sh
cargo fmt --all -- --check --verbose
cargo clippy --verbose -- -D warnings
```

8. If you've committed a change about command line arguments[*](https://github.com/orhun/menyoki/blob/master/src/args/mod.rs):
   * [Regenerate](https://github.com/orhun/menyoki/blob/master/completions/generate.sh) the shell completions. (run `./completions/generate.sh`)
   *  Update the [man pages](https://github.com/orhun/menyoki/tree/master/man), [configuration file](https://github.com/orhun/menyoki/blob/master/config/menyoki.conf) and [README.md](https://github.com/orhun/menyoki/blob/master/README.md) about these changes if necessary.

## Create a Pull Request

1. Ensure that you updated the documentation and filled the [Pull Request template](https://github.com/orhun/menyoki/blob/master/.github/PULL_REQUEST_TEMPLATE.md) according to the changes you made.

2. Wait for approval from the project owner/maintainer. Discuss the possible changes and update your Pull Request if necessary.

3. You may merge the Pull Request once you have the sign-off of the project owner/maintainer, or if you do not have permission to do that, you may request the project owner/maintainer to merge it in case s/he haven't done it after a while.

# License

By contributing, you agree that your contributions will be licensed under [GNU General Public License v3.0](https://github.com/orhun/menyoki/blob/master/LICENSE).
