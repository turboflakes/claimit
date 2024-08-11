use crate::providers::network::NetworkState;
use crate::providers::network::NetworkStatus;
use claimeer_common::runtimes::support::SupportedRelayRuntime;
use claimeer_common::types::{
    accounts::{Account, Balance},
    child_bounties::ChildBountyId,
    child_bounties::{ChildBounties, Filter, Id},
    claims::{ClaimState, ClaimStatus},
    extensions::ExtensionAccount,
    extensions::{ExtensionState, ExtensionStatus},
    layout::LayoutState,
};
use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use std::{
    env,
    str::FromStr,
    {collections::BTreeSet, rc::Rc},
};
use subxt::utils::AccountId32;
use yew::{Reducible, UseReducerHandle};

const ACCOUNTS_KEY: &str = "accounts";
const SIGNER_KEY: &str = "signer";

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct State {
    pub accounts: Vec<Account>,
    pub network: NetworkState,
    pub child_bounties_raw: Option<ChildBounties>,
    pub filter: Filter,
    pub extension: ExtensionState,
    pub claim: Option<ClaimState>,
    pub layout: LayoutState,
}

pub enum Action {
    /// Account actions
    AddAccount(String),
    RemoveAccountId(u32),
    DisableAccountId(u32),
    UpdateAccountIdBalance(u32, Balance),
    /// Claim actions
    StartClaim(Vec<Id>),
    SignClaim(ClaimState),
    SubmitClaim(ClaimState, Vec<u8>),
    CompleteClaim(ClaimState, Vec<ChildBountyId>),
    ResetClaim,
    ErrorClaim(ClaimState),
    /// Extension actions
    ConnectExtension,
    ChangeExtensionStatus(ExtensionStatus),
    ChangeSigner(ExtensionAccount),
    /// Network actions
    ChangeNetworkStatus(NetworkStatus),
    ChangeNetwork(SupportedRelayRuntime),
    UpdateBlockNumber(u32),
    UpdateChildBountiesRaw(ChildBounties),
    IncreaseFetch,
    /// Filter child bounties actions
    SetFilter(Filter),
    /// Layout actions
    ToggleLayoutAddAccountModal,
}

impl Reducible for State {
    type Action = Action;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            Action::AddAccount(address) => {
                let mut accounts = self.accounts.clone();
                // Verify if account is not already being followed
                if accounts
                    .iter()
                    .find(|&acc| acc.address == address)
                    .is_none()
                {
                    // Check if there are already some child bounties available to be linked
                    let child_bounty_ids =
                        if let Some(child_bounties) = self.child_bounties_raw.clone() {
                            child_bounties
                                .into_iter()
                                .filter(|(_, cb)| {
                                    let acc = AccountId32::from_str(&address).unwrap();
                                    acc == cb.beneficiary
                                })
                                .map(|(id, _)| id)
                                .collect::<BTreeSet<u32>>()
                        } else {
                            BTreeSet::new()
                        };

                    accounts.push(Account {
                        id: accounts.last().map(|account| account.id + 1).unwrap_or(1),
                        address,
                        identity: None,
                        disabled: false,
                        child_bounty_ids,
                        balance: Balance::new(),
                    });
                    LocalStorage::set(self.account_key(), accounts.clone()).expect("failed to set");
                }
                State {
                    accounts,
                    network: self.network.clone(),
                    child_bounties_raw: self.child_bounties_raw.clone(),
                    filter: self.filter.clone(),
                    extension: self.extension.clone(),
                    claim: self.claim.clone(),
                    layout: self.layout.clone(),
                }
                .into()
            }
            Action::RemoveAccountId(id) => {
                let mut accounts = self.accounts.clone();
                accounts.retain(|account| account.id != id);
                LocalStorage::set(self.account_key(), accounts.clone()).expect("failed to set");

                let following = accounts
                    .iter()
                    .map(|a| AccountId32::from_str(&a.address).unwrap())
                    .collect::<Vec<AccountId32>>();

                let filter = Filter::Following(following);

                State {
                    accounts,
                    network: self.network.clone(),
                    child_bounties_raw: self.child_bounties_raw.clone(),
                    filter,
                    extension: self.extension.clone(),
                    claim: self.claim.clone(),
                    layout: self.layout.clone(),
                }
                .into()
            }
            Action::DisableAccountId(id) => {
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
                    extension: self.extension.clone(),
                    claim: self.claim.clone(),
                    layout: self.layout.clone(),
                }
                .into()
            }
            Action::UpdateAccountIdBalance(id, balance) => {
                let mut accounts = self.accounts.clone();
                let account = accounts.iter_mut().find(|account| account.id == id);
                if let Some(account) = account {
                    account.balance = balance;
                }

                State {
                    accounts,
                    network: self.network.clone(),
                    child_bounties_raw: self.child_bounties_raw.clone(),
                    filter: self.filter.clone(),
                    extension: self.extension.clone(),
                    claim: self.claim.clone(),
                    layout: self.layout.clone(),
                }
                .into()
            }
            Action::StartClaim(child_bounty_ids) => {
                let claim = ClaimState::new(child_bounty_ids.clone());
                State {
                    accounts: self.accounts.clone(),
                    network: self.network.clone(),
                    child_bounties_raw: self.child_bounties_raw.clone(),
                    filter: self.filter.clone(),
                    extension: self.extension.clone(),
                    claim: Some(claim),
                    layout: self.layout.clone(),
                }
                .into()
            }
            Action::ResetClaim => State {
                accounts: self.accounts.clone(),
                network: self.network.clone(),
                child_bounties_raw: self.child_bounties_raw.clone(),
                filter: self.filter.clone(),
                extension: self.extension.clone(),
                claim: None,
                layout: self.layout.clone(),
            }
            .into(),
            Action::SignClaim(claim) => {
                let mut claim = claim.clone();
                claim.status = ClaimStatus::Signing;

                State {
                    accounts: self.accounts.clone(),
                    network: self.network.clone(),
                    child_bounties_raw: self.child_bounties_raw.clone(),
                    filter: self.filter.clone(),
                    extension: self.extension.clone(),
                    claim: Some(claim),
                    layout: self.layout.clone(),
                }
                .into()
            }
            Action::SubmitClaim(claim, tx_bytes) => {
                let mut claim = claim.clone();
                claim.status = ClaimStatus::Submitting(tx_bytes);

                State {
                    accounts: self.accounts.clone(),
                    network: self.network.clone(),
                    child_bounties_raw: self.child_bounties_raw.clone(),
                    filter: self.filter.clone(),
                    extension: self.extension.clone(),
                    claim: Some(claim),
                    layout: self.layout.clone(),
                }
                .into()
            }
            Action::CompleteClaim(claim, claimed) => {
                let mut claim = claim.clone();
                claim.status = ClaimStatus::Completed;

                let accounts = self
                    .accounts
                    .clone()
                    .into_iter()
                    .map(|account| {
                        let mut account = account.clone();
                        let ids = account
                            .child_bounty_ids
                            .into_iter()
                            .filter(|id| !claimed.contains(&(id)))
                            .collect::<BTreeSet<ChildBountyId>>();

                        account.child_bounty_ids = ids;
                        account
                    })
                    .collect::<Vec<Account>>();

                let child_bounties_raw =
                    if let Some(child_bounties) = self.child_bounties_raw.clone() {
                        child_bounties
                            .into_iter()
                            .filter(|(id, _)| !claimed.contains(&(id)))
                            .collect::<ChildBounties>()
                    } else {
                        ChildBounties::new()
                    };

                State {
                    accounts,
                    network: self.network.clone(),
                    child_bounties_raw: Some(child_bounties_raw),
                    filter: self.filter.clone(),
                    extension: self.extension.clone(),
                    claim: Some(claim),
                    layout: self.layout.clone(),
                }
                .into()
            }
            Action::ErrorClaim(claim) => {
                let mut claim = claim.clone();
                claim.status = ClaimStatus::Error;

                State {
                    accounts: self.accounts.clone(),
                    network: self.network.clone(),
                    child_bounties_raw: self.child_bounties_raw.clone(),
                    filter: self.filter.clone(),
                    extension: self.extension.clone(),
                    claim: Some(claim),
                    layout: self.layout.clone(),
                }
                .into()
            }
            Action::ConnectExtension => {
                let mut extension = self.extension.clone();
                extension.status = ExtensionStatus::Connecting;

                State {
                    accounts: self.accounts.clone(),
                    network: self.network.clone(),
                    child_bounties_raw: self.child_bounties_raw.clone(),
                    filter: self.filter.clone(),
                    extension,
                    claim: self.claim.clone(),
                    layout: self.layout.clone(),
                }
                .into()
            }
            Action::ChangeExtensionStatus(new_status) => {
                let mut extension = self.extension.clone();
                extension.status = new_status;

                State {
                    accounts: self.accounts.clone(),
                    network: self.network.clone(),
                    child_bounties_raw: self.child_bounties_raw.clone(),
                    filter: self.filter.clone(),
                    extension,
                    claim: self.claim.clone(),
                    layout: self.layout.clone(),
                }
                .into()
            }
            Action::ChangeSigner(account) => {
                LocalStorage::set(self.signer_key(), account.clone()).expect("failed to set");

                let mut extension = self.extension.clone();
                extension.signer = Some(account.clone());
                extension.status = ExtensionStatus::Ready;

                State {
                    accounts: self.accounts.clone(),
                    network: self.network.clone(),
                    child_bounties_raw: self.child_bounties_raw.clone(),
                    filter: self.filter.clone(),
                    extension,
                    claim: self.claim.clone(),
                    layout: self.layout.clone(),
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
                    extension: self.extension.clone(),
                    claim: self.claim.clone(),
                    layout: self.layout.clone(),
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
                    extension: self.extension.clone(),
                    claim: self.claim.clone(),
                    layout: self.layout.clone(),
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
                    extension: self.extension.clone(),
                    claim: self.claim.clone(),
                    layout: self.layout.clone(),
                }
                .into()
            }
            Action::IncreaseFetch => {
                let mut network = self.network.clone();
                network.fetches_counter += 1;
                State {
                    accounts: self.accounts.clone(),
                    network,
                    child_bounties_raw: self.child_bounties_raw.clone(),
                    filter: self.filter.clone(),
                    extension: self.extension.clone(),
                    claim: self.claim.clone(),
                    layout: self.layout.clone(),
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
                            let acc = AccountId32::from_str(&account.address).unwrap();
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
                    extension: self.extension.clone(),
                    claim: self.claim.clone(),
                    layout: self.layout.clone(),
                }
                .into()
            }
            Action::SetFilter(filter) => State {
                accounts: self.accounts.clone(),
                network: self.network.clone(),
                child_bounties_raw: self.child_bounties_raw.clone(),
                filter,
                extension: self.extension.clone(),
                claim: self.claim.clone(),
                layout: self.layout.clone(),
            }
            .into(),
            Action::ToggleLayoutAddAccountModal => {
                let mut layout = self.layout.clone();
                layout.is_add_account_modal_visible = !layout.is_add_account_modal_visible;

                State {
                    accounts: self.accounts.clone(),
                    network: self.network.clone(),
                    child_bounties_raw: self.child_bounties_raw.clone(),
                    filter: self.filter.clone(),
                    extension: self.extension.clone(),
                    claim: self.claim.clone(),
                    layout,
                }
                .into()
            }
        }
    }
}

impl State {
    pub fn account_key(&self) -> String {
        account_key(self.network.runtime)
    }

    pub fn signer_key(&self) -> String {
        signer_key(self.network.runtime)
    }
}

pub fn account_key(runtime: SupportedRelayRuntime) -> String {
    format!(
        "{}::{}::{}",
        env!("CARGO_PKG_NAME"),
        runtime.to_string().to_lowercase(),
        ACCOUNTS_KEY
    )
}

pub fn signer_key(runtime: SupportedRelayRuntime) -> String {
    format!(
        "{}::{}::{}",
        env!("CARGO_PKG_NAME"),
        runtime.to_string().to_lowercase(),
        SIGNER_KEY
    )
}

pub type StateContext = UseReducerHandle<State>;
