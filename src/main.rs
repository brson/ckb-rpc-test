/*
 *   Copyright (c) 2020 
 *   All rights reserved.
 */

#![allow(unused)]

use structopt::StructOpt;
use anyhow::Result;

mod account;

use crate::account::{Account, AccountName};

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
    name: AccountName,
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
    let _ = Account::create(&cmd.name);
    Ok(())
}
