# Substrate package manager

<img align="right" width="400" src="https://raw.githubusercontent.com/w3f/General-Grants-Program/79ea44570c6b8f10b286817a3d1a768d29810065/src/badge_black.svg"/>

[![etc](https://github.com/clearloop/sup/workflows/sup/badge.svg)](https://github.com/clearloop/sup)
[![crate](https://img.shields.io/crates/v/sup.svg)](https://crates.io/crates/sup)
[![downloads](https://img.shields.io/crates/d/sup.svg)](https://crates.io/crates/sup)
[![LICENSE](https://img.shields.io/crates/l/sup.svg)](https://choosealicense.com/licenses/apache-2.0/)

Sup is a package manager of [substrate][substrate] which uses the github as registry.

With `sup`, you can `create` you substrate node just in one command, `upgrade` or `downgrade`
your substrate dependencies in one command as well.

For the official tutorial of creating [substrate][substrate] node without sup, please check
out [Create Your First Substrate Chain][create-your-first-substrate-chain]. For the example
of using sup, please checkout the [GUIDE.md](./GUIDE.md).


## Installation

```bash
cargo install sup
```

## Usage

```bash
sup 0.1.10

USAGE:
    sup <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help       Prints this message or the help of the given subcommand(s)
    new        Create a new substrate node template
    source     List Source
    tag        List available tags
    update     Update registry
    upgrade    Upgrade substrate project
```

## Linked Projects

Welcome to [add][pr] your projects below if you've got a adventure of the substrate ecology
with sup!

+ [clearloop/cydonia][cydonia]

Sup has been recorded in [substrate-developer-hub/awesome-substrate][awesome], 
welcome to check other awesome projects listed in that repo.

## LICENSE

Apache-2.0

[awesome]: https://github.com/substrate-developer-hub/awesome-substrate#ecosystem-tools
[cydonia]: https://github.com/clearloop/sup/tree/w3f
[substrate]: https://github.com/paritytech/substrate
[pr]: https://github.com/clearloop/sup/pulls
[create-your-first-substrate-chain]: https://substrate.dev/docs/en/tutorials/create-your-first-substrate-chain/
