# Guide of Sup

Here we'll show the usage of Sup in detail.

+ Command [config](./customize-your-cargo-metadata)
+ Command [new](./create-a-new-node-template)
+ Command [list](./list-substrate-dependencies-by-tag)
+ Command [update](./update-substrate-dependencies)

## Create a node-template

First of all, download sup with `cargo install sup`, then you will get a binary
named sup in your terminal, just like this:

```text
$ sup
sup 0.2.7

USAGE:
    sup [FLAGS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -p, --pull       Updates the global registry
    -V, --version    Prints version information

SUBCOMMANDS:
    config    Shows or edits the current config
    help      Prints this message or the help of the given subcommand(s)
    list      List registry source or tags
    new       Create a new substrate node template
    update    Update the target substrate project
```

If everything works fine, let's create a `node-template` using sup:

```
$ sup new -h
sup-new 0.2.7
Create a new substrate node template

USAGE:
    sup new [FLAGS] [OPTIONS] <PATH>

FLAGS:
    -h, --help       Prints help information
    -s, --skip       Skip rust toolchain check
    -V, --version    Prints version information

OPTIONS:
    -t, --tag <tag>    Specify a tag to generate [default: v2.0.0]

ARGS:
    <PATH>    Project path
    
$ sup new cydonia
Created node-template "cydonia" succeed!

$ tree cydonia
cydonia
├── Cargo.toml
├── LICENSE
├── README.md
├── node
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── build.rs
│   └── src
│       ├── chain_spec.rs
│       ├── cli.rs
│       ├── command.rs
│       ├── lib.rs
│       ├── main.rs
│       ├── rpc.rs
│       └── service.rs
├── pallets
│   └── template
│       ├── Cargo.toml
│       ├── README.md
│       └── src
│           ├── lib.rs
│           ├── mock.rs
│           └── tests.rs
├── runtime
│   ├── Cargo.toml
│   ├── build.rs
│   └── src
│       └── lib.rs
└── scripts
    └── init.sh

8 directories, 22 files
```

Deal, our substrate node-template has been created!


## Customize your cargo metadata

The config file of sup is located at `~/.sup/config.toml`, by default, it's like:

```toml
[metadata]
authors = ["clearloop <udtrokia@gmail.com>"]
version = "0.1.0"
license = "MIT"

[node]
registry = "https://github.com/paritytech/substrate.git"

```

You can modify the registry by the shortcut `sup config set -r`,

```
sup-config 0.2.7
Shows or edits the current config

USAGE:
    sup config <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    edit    Edits the current config
    help    Prints this message or the help of the given subcommand(s)
    list    Lists the current config
    set     Sets config field
```

All info in metadata will invoke or replace into the node-template sup generates, 
And the `registry` determines where the context of our node-template comes from.


## list substrate dependencies by tag

Sometimes, we need to take a quick look at what dependencies the official substrate registry
offers, can `sup` help reach this? For example, now we want to find our how many `frame` the
substrate registry offers.

```
$ sup list
sup-list 0.2.7
List registry source or tags

USAGE:
    sup list <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    source    List source crates of registry
    tag       List tags of registry

$ sup list source | grep frame
frame-benchmarking                                 - 2.0.0
frame-benchmarking-cli                             - 2.0.0
frame-executive                                    - 2.0.0
frame-metadata                                     - 12.0.0
frame-support                                      - 2.0.0
frame-support-procedural                           - 2.0.0
frame-support-procedural-tools                     - 2.0.0
frame-support-procedural-tools-derive              - 2.0.0
frame-support-test                                 - 2.0.0
frame-system                                       - 2.0.0
frame-system-benchmarking                          - 2.0.0
frame-system-rpc-runtime-api                       - 2.0.0
substrate-frame-cli                                - 2.0.0
substrate-frame-rpc-support                        - 2.0.0
substrate-frame-rpc-system                         - 2.0.0
```

## Update substrate dependencies

Now we're going to switch a tag for our substrate project.

```
$ sup list tag
polkadot-0-8-25-dep
v2.0.0
v2.0.0-rc6
v2.0.0-rc5+2
v2.0.0-rc5+1
v2.0.0-rc5
v2.0.0-rc4
v2.0.0-rc3
v2.0.0-rc2
v2.0.0-rc2+2

$ sup update -h
 𝝺 ./target/debug/sup update -h
sup-update 0.2.7
Update the target substrate project

USAGE:
    sup update [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --project <project>    Project path [default: .]
    -t, --tag <tag>            Registry tag [default: ]

$ sup update -t v2.0.0-rc1
Upgrade "cydonia" succeed!
```

It works, we can use `switch` command to upgrade or downgrade our substrate
dependencies, don't forget the `-t` tag.
