# Guide of Sup

Here we'll show the usage of Sup in detail~

## Create a node-template

First of all, download sup with `cargo install sup`, then you will get a binary
named sup in your terminal, just like this:

```text
$ sup
sup 0.2.0

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

If everything works fine, let's create a `node-template` using sup:

```
$ sup create cydonia
$ tree cydonia
 𝝺 tree cydonia 
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


## Upgrade the substrate dependencies

