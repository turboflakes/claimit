use crate::components::{
    accounts::{AccountsCard, TotalBalancesCard},
    child_bounties::ChildBountiesCard,
    modals::{AddAccountModal, ClaimModal},
    nav::{Footer, Navbar},
    steps::OnboardingSteps,
};
use crate::router::Query;
use crate::state::{account_key, onboarded_key, signer_key, Action, State, StateContext};
use claimeer_common::runtimes::support::SupportedRelayRuntime;
use claimeer_common::types::{
    accounts::Account,
    child_bounties::Filter,
    claims::ClaimStatus,
    extensions::{ExtensionAccount, ExtensionState},
    layout::LayoutState,
    network::{NetworkState, NetworkStatus},
};
use claimeer_workers::network_api::{Input as WorkerInput, Output as WorkerOutput, Worker};
use gloo::storage::{LocalStorage, Storage};
use std::str::FromStr;
use subxt::config::substrate::AccountId32;
use yew::{
    classes, function_component, html, prelude::use_reducer, use_callback, use_effect_with,
    ContextProvider, Html,
};
use yew_agent::reactor::{
    use_reactor_bridge, ReactorEvent, ReactorProvider, UseReactorBridgeHandle,
};
use yew_router::prelude::use_location;

#[function_component(Main)]
pub fn main() -> Html {
    let location = use_location().unwrap();
    let current_runtime = location
        .query::<Query>()
        .map(|q| q.chain)
        .unwrap_or(SupportedRelayRuntime::Polkadot);

    let state = use_reducer(|| {
        let accounts: Vec<Account> =
            LocalStorage::get(account_key(current_runtime.clone())).unwrap_or_else(|_| vec![]);

        let is_onboarding = accounts.len() == 0
            || !LocalStorage::get(onboarded_key(current_runtime.clone())).unwrap_or(false);

        let filter = match is_onboarding {
            true => Filter::All,
            false => {
                let following = accounts
                    .iter()
                    .map(|a| AccountId32::from_str(&a.address).unwrap())
                    .collect::<Vec<AccountId32>>();
                Filter::Following(following)
            }
        };

        let signer: Option<ExtensionAccount> =
            LocalStorage::get(signer_key(current_runtime.clone())).unwrap_or_default();

        State {
            accounts,
            network: NetworkState::new(current_runtime.clone(), false),
            child_bounties_raw: None,
            filter,
            extension: ExtensionState::new(signer.clone()),
            claim: None,
            layout: LayoutState::new(is_onboarding),
        }
    });

    // Handle api calls over bridge (all async network calls are handled in a specific web worker)
    let worker_api_bridge: UseReactorBridgeHandle<Worker> = use_reactor_bridge({
        let state = state.clone();
        move |response| match response {
            ReactorEvent::Output(output) => match output {
                WorkerOutput::Active(_sub_id) => {
                    state.dispatch(Action::ChangeNetworkStatus(NetworkStatus::Active));
                }
                WorkerOutput::BlockNumber(sub_id, block_number) => {
                    if state.network.subscription_id == sub_id {
                        state.dispatch(Action::UpdateBlockNumber(block_number));
                    }
                }
                WorkerOutput::ChildBounties(data) => {
                    state.dispatch(Action::UpdateChildBountiesRaw(data))
                }
                WorkerOutput::AccountBalance(account, balance) => {
                    state.dispatch(Action::UpdateAccountBalance(account, balance));
                }
                WorkerOutput::TxPayload(payload) => {
                    state.dispatch(Action::GetSignature(payload));
                }
                WorkerOutput::TxCompleted(child_bounties_ids) => {
                    state.dispatch(Action::CompleteClaim(child_bounties_ids));
                }
                WorkerOutput::Err(_) => {
                    state.dispatch(Action::ChangeNetworkStatus(NetworkStatus::Inactive));
                }
            },
            ReactorEvent::Finished => {}
        }
    });

    // Fetch initial data when network changes and is active or light client ready
    use_effect_with(state.network.status.clone(), {
        let state = state.clone();
        let worker_api_bridge = worker_api_bridge.clone();
        move |status| match status {
            NetworkStatus::Initializing => {
                worker_api_bridge.send(WorkerInput::Start(
                    state.network.subscription_id,
                    state.network.runtime,
                    state.network.use_light_client_as_network_provider,
                ));
            }
            NetworkStatus::Active => {
                state.dispatch(Action::IncreaseFetch);
                worker_api_bridge.send(WorkerInput::FetchChildBounties);
                for account in &state.accounts {
                    let acc = AccountId32::from_str(&account.address).unwrap();
                    worker_api_bridge.send(WorkerInput::FetchAccountBalance(acc));
                }
            }
            _ => (),
        }
    });

    // Fetch accounts balance everytime there is a change on the accounts being followed
    use_effect_with(state.accounts.clone(), {
        let state = state.clone();
        let worker_api_bridge = worker_api_bridge.clone();
        move |accounts| {
            if accounts.len() == 0 {
                state.dispatch(Action::StartOnboarding);
            }
            for account in &state.accounts {
                let acc = AccountId32::from_str(&account.address).unwrap();
                worker_api_bridge.send(WorkerInput::FetchAccountBalance(acc));
            }
        }
    });

    //
    use_effect_with(state.claim.clone(), {
        let state = state.clone();
        let worker_api_bridge = worker_api_bridge.clone();
        let extension = state.extension.clone();
        move |claim| {
            if let Some(claim) = claim {
                match &claim.status {
                    ClaimStatus::Preparing => {
                        if extension.is_ready() {
                            let signer = extension.signer.as_ref().unwrap().clone();
                            let claim = claim.clone();
                            worker_api_bridge.send(WorkerInput::CreatePayloadTx(
                                claim.child_bounty_ids.clone(),
                                signer.address.clone(),
                            ));
                        }
                    }
                    ClaimStatus::Submitting(signature) => {
                        if extension.is_ready() {
                            let signer = extension.signer.as_ref().unwrap().clone();
                            let claim = claim.clone();
                            worker_api_bridge.send(WorkerInput::SignAndSubmitTx(
                                claim.child_bounty_ids.clone(),
                                signer.address.clone(),
                                signature.clone(),
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }
    });

    let onchange_network = use_callback(
        (state.clone(), worker_api_bridge.clone()),
        |runtime, (state, worker_api_bridge)| {
            worker_api_bridge.send(WorkerInput::Finish);
            worker_api_bridge.reset();
            // Note: Reset network with default to the current provider
            state.dispatch(Action::ResetNetwork(
                runtime,
                state.network.use_light_client_as_network_provider,
            ));
        },
    );

    let ontoggle_provider = use_callback(
        (state.clone(), worker_api_bridge.clone()),
        |_, (state, worker_api_bridge)| {
            worker_api_bridge.send(WorkerInput::Finish);
            worker_api_bridge.reset();
            // Note: Reset network toggle current provider
            state.dispatch(Action::ResetNetwork(
                state.network.runtime.clone(),
                !state.network.use_light_client_as_network_provider,
            ));
        },
    );

    let hidden_class = if state.layout.is_onboarding {
        Some("flex")
    } else {
        Some("hidden sm:flex")
    };

    html! {
        <ContextProvider<StateContext> context={state.clone()}>
            <div class={classes!("main", current_runtime.class())}>
                <Navbar runtime={current_runtime.clone()} ontoggle_provider={&ontoggle_provider} />

                <div class="grid grid-cols-1 sm:grid-cols-3">

                    <div class="flex flex-col justify-center items-center px-2 sm:px-4 sm:h-screen">

                        <div class={classes!("header", hidden_class)}>
                            <img class="mb-4 sm:mb-8 max-w-[160px] sm:max-w-[256px]" src="/images/claimeer_logo.svg" alt="Claimeer" />
                            <p class="text-md sm:text-xl text-light text-center tracking-wide text-gray-900">{"Claim Child Bounties with Ease!"}</p>
                        </div>

                        <div class="hidden sm:flex w-full">
                            <Footer runtime={current_runtime.clone()} onchange={&onchange_network} />
                        </div>

                    </div>

                    <div class="sm:col-span-2">

                        <div class="flex-auto sm:h-screen w-full overflow-hidden sm:overflow-auto">

                            <div class="flex flex-col items-center my-4 mt-20 sm:mt-32 px-2 sm:px-4 sm:px-0">

                                {
                                    if state.layout.is_onboarding {
                                        html! {
                                            <OnboardingSteps />
                                        }
                                    } else {
                                        html! {
                                            <>
                                                <TotalBalancesCard runtime={current_runtime.clone()} />
                                                <AccountsCard runtime={current_runtime.clone()} />
                                                <ChildBountiesCard />
                                            </>
                                        }
                                    }
                                }

                            </div>

                        </div>

                    </div>

                    <div class="sm:hidden w-full">
                        <Footer runtime={current_runtime.clone()} onchange={&onchange_network} />
                    </div>

                </div>

                <ClaimModal />
                <AddAccountModal />
            </div>
        </ContextProvider<StateContext>>
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <ReactorProvider<Worker> path="/network_api_worker.js">
            <Main />
        </ReactorProvider<Worker>>
    }
}
