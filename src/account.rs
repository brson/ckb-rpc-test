use anyhow::Result;
use ckb_sdk::wallet::{MasterPrivKey, Key};

use crate::GlobalOpts;

pub type AccountName = String;

pub struct Account {
    key: Key,
}

impl Account {
    pub fn create(opts: &GlobalOpts, name: &AccountName) -> Result<Account> {
        let privkey = MasterPrivKey::try_new(1024)?;
        let key = Key::new(privkey);
        Ok(Account { key })
    }

    pub fn load(opts: &GlobalOpts, name: &AccountName) -> Result<Account> {
        panic!()
    }
}

