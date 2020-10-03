# Devup #

Devup is a swiss-army tool for my day-to-day development flow. It's got a few commands to setup a new environment, as well as some maintenance ones.

## Config ##

- Powered by [confy](https://github.com/rust-cli/confy), you can find the TOML-styled config for tweaking in your system's specific config directory (as determined by the [directories](https://crates.io/crates/directories). For MacOS this is `~/Library/Preferences/rs.devup/devup.toml`.

## Arugments ##

Run help for the latest...

- `setup`: Config this tool
- `inis`: Copy env configs to local machine from dev env
- `list`: List branches you own for current repo
- `certs`: Copy certs to local machine from dev env