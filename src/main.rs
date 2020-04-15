/*
 *   Copyright (c) 2020 
 *   All rights reserved.
 */

#![allow(unused)]

use structopt::StructOpt;
use anyhow::Result;

#[derive(StructOpt)]
struct Opts {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(StructOpt)]
enum Command {
}

fn main() -> Result<()> {
    let opts = Opts::from_args();
    panic!()
}
