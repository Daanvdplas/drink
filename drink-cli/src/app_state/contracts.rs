use std::path::PathBuf;

use contract_transcode::ContractMessageTranscoder;
use sp_core::crypto::AccountId32;
use ContractIndex::NoContracts;

use crate::app_state::ContractIndex::CurrentContract;

pub struct Contract {
    pub name: String,
    pub address: AccountId32,
    pub base_path: PathBuf,
    pub transcode: ContractMessageTranscoder,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum ContractIndex {
    #[default]
    NoContracts,
    CurrentContract(usize),
}

#[derive(Default)]
pub struct ContractRegistry {
    contracts: Vec<Contract>,
    index: ContractIndex,
}

impl ContractRegistry {
    pub fn add(&mut self, contract: Contract) {
        self.contracts.push(contract);
        self.index = CurrentContract(self.contracts.len() - 1);
    }

    pub fn current_index(&self) -> ContractIndex {
        self.index
    }

    pub fn current_contract(&self) -> Option<&Contract> {
        match self.index {
            NoContracts => None,
            CurrentContract(idx) => Some(&self.contracts[idx]),
        }
    }

    pub fn get_all(&self) -> &[Contract] {
        &self.contracts
    }

    pub fn next(&mut self) -> Option<&Contract> {
        let CurrentContract(old_index) = self.index else {
            return None;
        };

        self.index = CurrentContract((old_index + 1) % self.contracts.len());
        self.current_contract()
    }
}
