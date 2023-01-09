![built with Svelte](https://img.shields.io/badge/Svelte-4A4A55?style=for-the-badge&logo=svelte&logoColor=FF3E00)
![built with Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![built with SQLite](https://img.shields.io/badge/SQLite-07405E?style=for-the-badge&logo=sqlite&logoColor=white)
[![built with nix](https://builtwithnix.org/badge.svg)](https://builtwithnix.org)

# how to setup
(sorted in order of less painful to more painful)

## I run x86 Linux and I have [Nix](https://nixos.org/download.html#nix-install-linux) installed
- depending on when you're reading this, you might need to [do this](https://nixos.wiki/wiki/Flakes#Enable_flakes)
- install dependencies automagically with `nix develop`
- run in near-production environment with `nix run`

## I run Windows
- install [WSL](https://learn.microsoft.com/en-us/windows/wsl/install)
- install [Nix](https://nixos.org/download.html#nix-install-windows) in WSL environment
- follow above instructions

## I run Windows but I don't want to deal with WSL
- install [NodeJS](https://nodejs.org/en/download/)
- install [Rust](https://www.rust-lang.org/tools/install)
- install [Yarn](https://classic.yarnpkg.com/lang/en/docs/install)

## I run something else
- ¯\\\_(ツ)\_\/¯
- let me know and I'll try my best to help

# how to run
run `cargo run` in the base folder

# deployment notes
- deploy with `deploy` ([deploy-rs](https://github.com/serokell/deploy-rs)) inside the `nix develop` environment
- required authorized SSH key
- to set email for ACME place it in `.letsencrypt_email.txt`, then run `git add --intent-to-add .letsencrypt_email.txt -f && git update-index --assume-unchanged .letsencrypt_email.txt`
- frustrating out of memory sigterm occurs if less than 1GB of RAM (if using remote build)

# sqlite notes
sqlite will actually ignore a lot of things that one might initially assume it shouldn't, examples include:
- `VARCHAR(n)`: ignored, it's actually just `TEXT` (use `CHECK` instead)
- `REFERENCES`: ignored, use `FOREIGN KEY` instead

for some reason using the `bundled` feature for `libsqlite3-sys` makes it ignore `PRAGMA legacy_alter_table = on;`
