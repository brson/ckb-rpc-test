use anyhow::Result;

pub type AccountName = String;

pub struct Account;

impl Account {
    pub fn create(name: &AccountName) -> Result<Account> {
        panic!()
    }

    pub fn load(name: &AccountName) -> Result<Account> {
        panic!()
    }
}

