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
    CreateAccount(CreateAccountCommand),
}

#[derive(StructOpt)]
struct CreateAccountCommand {
}

fn main() -> Result<()> {
    let opts = Opts::from_args();

    match opts.command {
        Command::CreateAccount(cmd) => {
            create_account(cmd)
        }
    }
}

fn create_account(cmd: CreateAccountCommand) -> Result<()> {
    panic!()
}
