use crate::providers::network::NetworkStatus;
use crate::types::{
    accounts::Account,
    child_bounties::{ChildBounties, Filter},
};
use crate::{providers::network::NetworkState, runtimes::support::SupportedRelayRuntime};
use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::{collections::BTreeSet, rc::Rc};
use subxt::utils::AccountId32;
use yew::{Reducible, UseReducerHandle};

const ACCOUNTS_KEY: &str = "accounts";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct State {
    pub accounts: Vec<Account>,
    pub network: NetworkState,
    pub child_bounties_raw: Option<ChildBounties>,
    pub filter: Filter,
}

pub enum Action {
    /// Account actions
    Add(String),
    Remove(usize),
    Toggle(usize),
    /// Network actions
    ChangeNetworkStatus(NetworkStatus),
    ChangeNetwork(SupportedRelayRuntime),
    UpdateBlockNumber(u32),
    UpdateChildBountiesRaw(ChildBounties),
    AddFetch,
    // Filter Child Bounties
    SetFilter(Filter),
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::Add(ss58) => {
                let mut accounts = self.accounts.clone();
                // Verify if account is not already being followed
                if accounts.iter().find(|&acc| acc.ss58 == ss58).is_none() {
                    // Check if there are already some child bounties available to be linked
                    let child_bounty_ids =
                        if let Some(child_bounties) = self.child_bounties_raw.clone() {
                            child_bounties
                                .into_iter()
                                .filter(|(_, cb)| {
                                    let acc = AccountId32::from_str(&ss58).unwrap();
                                    acc == cb.beneficiary
                                })
                                .map(|(id, _)| id)
                                .collect::<BTreeSet<u32>>()
                        } else {
                            BTreeSet::new()
                        };

                    accounts.push(Account {
                        id: accounts.last().map(|account| account.id + 1).unwrap_or(1),
                        ss58,
                        identity: None,
                        disabled: false,
                        child_bounty_ids,
                    });
                    LocalStorage::set(self.account_key(), accounts.clone()).expect("failed to set");
                }
                State {
                    accounts,
                    network: self.network.clone(),
                    child_bounties_raw: self.child_bounties_raw.clone(),
                    filter: self.filter.clone(),
                }
                .into()
            }
            Action::Remove(id) => {
                let mut accounts = self.accounts.clone();
                accounts.retain(|account| account.id != id);
                LocalStorage::set(self.account_key(), accounts.clone()).expect("failed to set");
                State {
                    accounts,
                    network: self.network.clone(),
                    child_bounties_raw: self.child_bounties_raw.clone(),
                    filter: self.filter.clone(),
                }
                .into()
            }
            Action::Toggle(id) => {
                let mut accounts = self.accounts.clone();
                let account = accounts.iter_mut().find(|account| account.id == id);
                if let Some(account) = account {
                    account.disabled = !account.disabled;
                }
                State {
                    accounts,
                    network: self.network.clone(),
                    child_bounties_raw: self.child_bounties_raw.clone(),
                    filter: self.filter.clone(),
                }
                .into()
            }
            Action::UpdateBlockNumber(block_number) => {
                let mut network = self.network.clone();
                network.finalized_block_number = Some(block_number);

                State {
                    accounts: self.accounts.clone(),
                    network,
                    child_bounties_raw: self.child_bounties_raw.clone(),
                    filter: self.filter.clone(),
                }
                .into()
            }
            Action::ChangeNetworkStatus(new_status) => {
                let mut network = self.network.clone();
                network.status = new_status;
                State {
                    accounts: self.accounts.clone(),
                    network,
                    child_bounties_raw: self.child_bounties_raw.clone(),
                    filter: self.filter.clone(),
                }
                .into()
            }
            Action::ChangeNetwork(runtime) => {
                let network = NetworkState::new(runtime.clone());
                let accounts =
                    LocalStorage::get(account_key(runtime.clone())).unwrap_or_else(|_| vec![]);
                State {
                    accounts,
                    network,
                    child_bounties_raw: None,
                    filter: self.filter.clone(),
                }
                .into()
            }
            Action::AddFetch => {
                let mut network = self.network.clone();
                network.fetches_counter += 1;
                State {
                    accounts: self.accounts.clone(),
                    network,
                    child_bounties_raw: self.child_bounties_raw.clone(),
                    filter: self.filter.clone(),
                }
                .into()
            }
            Action::UpdateChildBountiesRaw(data) => {
                let mut network = self.network.clone();
                network.fetches_counter -= 1;
                // Filter and Map the child bounties against the accounts being tracked
                let mut accounts = self.accounts.clone();
                for account in accounts.iter_mut() {
                    account.child_bounty_ids = BTreeSet::new();

                    let ids = data
                        .clone()
                        .into_iter()
                        .filter(|(_, cb)| {
                            let acc = AccountId32::from_str(&account.ss58).unwrap();
                            acc == cb.beneficiary
                        })
                        .map(|(id, _)| id)
                        .collect::<BTreeSet<u32>>();

                    for id in ids {
                        account.child_bounty_ids.insert(id.clone());
                    }
                }
                LocalStorage::set(self.account_key(), accounts.clone()).expect("failed to set");

                State {
                    accounts,
                    network,
                    child_bounties_raw: Some(data.clone()),
                    filter: self.filter.clone(),
                }
                .into()
            }
            Action::SetFilter(filter) => State {
                accounts: self.accounts.clone(),
                network: self.network.clone(),
                child_bounties_raw: self.child_bounties_raw.clone(),
                filter,
            }
            .into(),
        }
    }
}

impl State {
    pub fn account_key(&self) -> String {
        account_key(self.network.runtime)
    }
}

pub fn account_key(runtime: SupportedRelayRuntime) -> String {
    format!("{}:{}", runtime.to_string(), ACCOUNTS_KEY)
}

pub type StateContext = UseReducerHandle<State>;
