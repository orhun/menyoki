# Creating a Release <a href="https://github.com/orhun/menyoki"><img src="https://user-images.githubusercontent.com/24392180/99184076-96c10b00-2751-11eb-99ea-ad962144df76.png" width="25"></a>

[GitHub](https://github.com/orhun/menyoki/releases), [crates.io](https://crates.io/crates/menyoki/) and [Docker](https://hub.docker.com/r/orhunp/menyoki) releases are automated via [GitHub actions](https://github.com/orhun/menyoki/blob/master/.github/workflows/cd.yml) and triggered by pushing a tag.

1. Bump the version in [Cargo.toml](https://github.com/orhun/menyoki/blob/master/Cargo.toml) according to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).
2. Update [Cargo.lock](https://github.com/orhun/menyoki/blob/master/Cargo.lock) by building the project via `cargo build`.
3. Ensure [CHANGELOG.md](https://github.com/orhun/menyoki/blob/master/CHANGELOG.md) is updated according to [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) format.
4. Commit and push the changes.
5. Check if [Continuous Integration](https://github.com/orhun/menyoki/actions) workflow is completed successfully.
6. Create a new tag: `git tag -s -a v[x.y.z]` ([signed](https://keyserver.ubuntu.com/pks/lookup?search=0x53F218C35C1DC8B1&op=vindex))
7. Push the tag: `git push --tags`
8. Wait for [Continuous Deployment](https://github.com/orhun/menyoki/actions) workflow to finish.

### Arch Linux

Flag the packages out-of-date both on [AUR](https://aur.archlinux.org/packages/?O=0&SeB=b&K=menyoki&outdated=&SB=n&SO=a&PP=50&do_Search=Go) and [community repository](https://archlinux.org/packages/community/x86_64/menyoki/). If they are not updated within 3 days, contact and ask for the [current maintainer](mailto:orhunparmaksiz@gmail.com) to update them.
