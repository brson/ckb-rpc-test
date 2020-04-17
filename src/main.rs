/*
 *   Copyright (c) 2020 
 *   All rights reserved.
 */

#![allow(unused)]

use structopt::StructOpt;
use anyhow::Result;
use std::path::PathBuf;

mod account;

use crate::account::{Account, AccountName};

#[derive(StructOpt)]
struct Opts {
    #[structopt(subcommand)]
    command: Command,
    #[structopt(flatten)]
    global_opts: GlobalOpts,
}

#[derive(StructOpt)]
enum Command {
    CreateAccount(CreateAccountCommand),
}

#[derive(StructOpt)]
struct CreateAccountCommand {
    name: AccountName,
}

#[derive(StructOpt)]
pub struct GlobalOpts {
    #[structopt(default_value = "./data/")]
    data_path: PathBuf,
}

fn main() -> Result<()> {
    let opts = Opts::from_args();

    match opts.command {
        Command::CreateAccount(cmd) => {
            create_account(opts.global_opts, cmd)
        }
    }
}

fn create_account(opts: GlobalOpts, cmd: CreateAccountCommand) -> Result<()> {
    let account = Account::create(&opts, &cmd.name)?;

    println!("account created");
    println!("name: {}", account.name);
    println!("lock_arg: {}", account.lock_arg_string());
    println!("lock_hash: {}", account.lock_hash_string());
    println!("mainnet address: {}", account.mainnet_address_string());
    println!("testnet address: {}", account.testnet_address_string());

    Ok(())
}
