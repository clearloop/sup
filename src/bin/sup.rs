// src/main.rs
// use spm::cmds::Opt;
// use structopt::StructOpt;
use git2::Repository;

fn main() {
    let url = "https://github.com/paritytech/substrate.git";
    let _repo = match Repository::clone(url, "./substrate") {
        Ok(repo) => repo,
        Err(e) => panic!("failed to clone: {}", e),
    };

    // println!("{:#?}", repo);
}
