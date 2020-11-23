# Guide of Sup

Here we'll show the usage of Sup in detail~

## Create a node-template

First of all, download sup with `cargo install sup`, then you will get a binary
named sup in your terminal, just like this:

```text
$ sup
sup 0.2.6

USAGE:
    sup <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    config    Show or edit the current config
    help      Prints this message or the help of the given subcommand(s)
    new       Create a new substrate node template
    source    List source crates
    switch    Switch registry tag for the target substrate project
    tag       List available tags
    update    Update registry
```

If everything works fine, let's create a `node-template` using sup:

```
$ sup new -h
sup-new 0.2.6
Create a new substrate node template

USAGE:
    sup new [FLAGS] [OPTIONS] <PATH>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --tag <tag>    Specify a tag to generate [default: ]

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


## Downupgrade the substrate dependencies

Now we're going to switch a tag for our substrate project.

```
 $ sup tag -h
sup-tag 0.2.6
List available tags

USAGE:
    sup tag [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -u, --update     If update registry
    -V, --version    Prints version information

OPTIONS:
    -l, --limit <limit>    The limit count of tags [default: 10]
    
$ sup tag
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

$ sup upgrade -t v2.0.0-rc1
Upgrade "cydonia" succeed!
```

It works, we can use `switch` command to upgrade or downgrade our substrate
dependencies, don't forget the `-t` tag.


## Select a new substrate dependency

Sometimes, we need to take a quick look at what dependencies the official substrate registry
offers, can `sup` help reach this? For example, now we want to find our how many `frame` the
substrate registry offers.

```
$ sup source -h
sup-source 0.2.6
List source crates

USAGE:
    sup source [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -v, --version    If show versions

OPTIONS:
    -q, --query <query>    Show dependencies contain <query> [default: ]
    -t, --tag <tag>        Registry tag [default: ]

$ sup source -q frame
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
