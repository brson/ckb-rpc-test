use std::fs::{self, File};
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};
use ckb_sdk::wallet::{MasterPrivKey, Key};
use ckb_sdk::{NetworkType, Address, AddressPayload};
use ckb_types::{H160, H256};
use ckb_types::packed::Script;
use ckb_types::prelude::Unpack;
use ckb_sdk::wallet::ScryptType;

use crate::GlobalOpts;

pub type AccountName = String;

pub struct Account {
    pub name: AccountName,
    pub key: Key,
    pub address_payload: AddressPayload,
    pub lock_hash: H256,
    pub mainnet_address: Address,
    pub testnet_address: Address,
}

impl Account {
    pub fn create(opts: &GlobalOpts, name: &AccountName) -> Result<Account> {
        use failure::ResultExt;

        let name = name.clone();
        let privkey = MasterPrivKey::try_new(1024).compat()?;
        let key = Key::new(privkey);
        let account = Account::from_key(key, name);
        account.save(opts)?;
        Ok(account)
    }

    fn from_key(key: Key, name: AccountName) -> Account {
        let lock_arg = key.hash160();
        let address_payload = AddressPayload::from_pubkey_hash(lock_arg.clone());
        let lock_hash: H256 = Script::from(&address_payload).calc_script_hash().unpack();
        let mainnet_address = Address::new(NetworkType::Mainnet, address_payload.clone());
        let testnet_address = Address::new(NetworkType::Testnet, address_payload.clone());
        Account {
            name,
            key, address_payload, lock_hash,
            mainnet_address, testnet_address,
        }
    }

    fn save(&self, opts: &GlobalOpts) -> Result<()> {
        let dir_path = &opts.data_dir;
        let account_file = dir_path.join(format!("{}.json", &self.name));
        fs::create_dir_all(dir_path)
            .context("failed to create data directory")?;
        let mut file = File::create(&account_file)
            .context("failed to open account file")?;
        let key = self.key.to_json(&[], ScryptType::Light);
        serde_json::to_writer_pretty(file, &key)
            .context("failed to write account file")?;
        Ok(())
    }

    pub fn load(opts: &GlobalOpts, name: &AccountName) -> Result<Account> {
        let dir_path = &opts.data_dir;
        let account_file = dir_path.join(format!("{}.json", &name));
        let mut file = File::create(&account_file)
            .context("failed to open account file")?;
        let json_key = serde_json::from_reader(file)
            .context("failed to read account file")?;
        fn deserialize(json: serde_json::Value) -> Result<Key> {
            use failure::ResultExt;
            let key = Key::from_json(&json, &[]).compat()?;
            Ok(key)
        }
        let key = deserialize(json_key)?;
        Ok(Account::from_key(key, name.to_string()))
    }

    /// The public identifier of the account key
    pub fn lock_arg(&self) -> &H160 {
        self.key.hash160()
    }

    pub fn lock_arg_string(&self) -> String {
        format!("{:#x}", self.lock_arg())
    }

    pub fn lock_hash_string(&self) -> String {
        format!("{:#x}", self.lock_hash)
    }

    pub fn mainnet_address_string(&self) -> String {
        self.mainnet_address.to_string()
    }

    pub fn testnet_address_string(&self) -> String {
        self.testnet_address.to_string()
    }
}

