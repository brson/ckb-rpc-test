use anyhow::Result;

use crate::GlobalOpts;

pub type AccountName = String;

pub struct Account;

impl Account {
    pub fn create(opts: &GlobalOpts, name: &AccountName) -> Result<Account> {
        panic!()
    }

    pub fn load(opts: &GlobalOpts, name: &AccountName) -> Result<Account> {
        panic!()
    }
}

