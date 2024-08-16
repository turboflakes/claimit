use crate::providers::network::NetworkStatus;
use crate::router::Query;
use crate::state::{account_key, onboarded_key, signer_key, Action, State, StateContext};
use crate::workers::{
    network_storage::{Query as StorageQuery, Response as StorageResponse, StorageQueries},
    network_subscription::{
        BlockSubscription, Input as SubscriptionInput, Output as SubscriptionOutput,
    },
};
use crate::{
    components::{
        accounts::{AccountsCard, TotalBalancesCard},
        child_bounties::ChildBountiesCard,
        modals::{AddAccountModal, ClaimModal},
        nav::{Footer, Navbar},
        steps::OnboardingSteps,
    },
    providers::network::NetworkState,
};
use claimeer_common::runtimes::support::SupportedRelayRuntime;
use claimeer_common::types::{
    accounts::Account, child_bounties::Filter, extensions::ExtensionAccount,
    extensions::ExtensionState, layout::LayoutState,
};
use gloo::storage::{LocalStorage, Storage};
use log::debug;
use std::str::FromStr;
use subxt::config::substrate::AccountId32;
use yew::{
    classes, function_component, html, platform::spawn_local, prelude::use_reducer, use_callback,
    use_effect_with, ContextProvider, Html,
};
use yew_agent::{
    oneshot::{use_oneshot_runner, OneshotProvider},
    reactor::{use_reactor_bridge, ReactorEvent, ReactorProvider, UseReactorBridgeHandle},
};
use yew_router::prelude::use_location;

#[function_component(Main)]
pub fn main() -> Html {
    let location = use_location().unwrap();
    let current_runtime = location
        .query::<Query>()
        .map(|q| q.chain)
        .unwrap_or(SupportedRelayRuntime::Polkadot);

    let storage_task = use_oneshot_runner::<StorageQueries>();

    // State
    let state = use_reducer(|| {
        let accounts: Vec<Account> =
            LocalStorage::get(account_key(current_runtime.clone())).unwrap_or_else(|_| vec![]);

        let is_onboarding = accounts.len() == 0
            || !LocalStorage::get(onboarded_key(current_runtime.clone())).unwrap_or(false);

        let following = accounts
            .iter()
            .map(|a| AccountId32::from_str(&a.address).unwrap())
            .collect::<Vec<AccountId32>>();

        let filter = match is_onboarding {
            true => Filter::All,
            false => Filter::Following(following),
        };

        let signer: Option<ExtensionAccount> =
            LocalStorage::get(signer_key(current_runtime.clone())).unwrap_or_default();

        State {
            accounts,
            network: NetworkState::new(current_runtime.clone()),
            child_bounties_raw: None,
            filter,
            extension: ExtensionState::new(signer.clone()),
            claim: None,
            layout: LayoutState::new(is_onboarding),
        }
    });

    use_effect_with(state.accounts.clone(), {
        let state = state.clone();
        move |accounts| {
            if accounts.len() == 0 {
                state.dispatch(Action::StartOnboarding);
            }
        }
    });

    // Handle subscription responses over bridge
    let subscription_bridge: UseReactorBridgeHandle<BlockSubscription> = use_reactor_bridge({
        let state = state.clone();
        move |response| match response {
            ReactorEvent::Output(output) => match output {
                SubscriptionOutput::Active(_sub_id) => {
                    state.dispatch(Action::ChangeNetworkStatus(NetworkStatus::Active));

                    // Query all child bounties
                    let storage_task = storage_task.clone();
                    let runtime = current_runtime.clone();

                    let state = state.clone();
                    spawn_local(async move {
                        state.dispatch(Action::IncreaseFetch);
                        let response = storage_task
                            .run(StorageQuery::FetchChildBounties(runtime))
                            .await;
                        match response {
                            StorageResponse::ChildBounties(data) => {
                                state.dispatch(Action::UpdateChildBountiesRaw(data))
                            }
                        };
                    });
                }
                SubscriptionOutput::BlockNumber(sub_id, block_number) => {
                    if state.network.subscription_id == sub_id {
                        state.dispatch(Action::UpdateBlockNumber(block_number));
                    }
                }
                SubscriptionOutput::Err(_) => {
                    state.dispatch(Action::ChangeNetworkStatus(NetworkStatus::Inactive));
                }
            },
            ReactorEvent::Finished => debug!("subscription finished"),
        }
    });

    // Start subscription
    subscription_bridge.send(SubscriptionInput::Start(
        state.network.subscription_id,
        state.network.runtime,
    ));

    let onchange_network = use_callback(
        (state.clone(), subscription_bridge.clone()),
        |runtime, (state, subscription_bridge)| {
            subscription_bridge.send(SubscriptionInput::Finish);
            subscription_bridge.reset();
            state.dispatch(Action::ChangeNetwork(runtime));
        },
    );

    let _hidden_class = if state.accounts.is_empty() {
        "hidden"
    } else {
        ""
    };

    // Html
    html! {
        <>
            <div class={classes!("main", current_runtime.class())}>
                <ContextProvider<StateContext> context={state.clone()}>

                    <Navbar runtime={current_runtime.clone()} />

                    <div class="grid grid-cols-1 sm:grid-cols-3">

                        <div class="px-4 flex flex-col justify-center items-center h-screen">
                            // {
                            //     if state.accounts.len() > 0 {
                            //         html! {
                                        <div class="flex flex-col flex-1 justify-center items-center">
                                            <img class="mb-8 max-w-[256px]" src="/images/claimeer_logo.svg" alt="Claimeer logo" />
                                            <p class="text-xl text-light text-center tracking-wide text-gray-900">{"Secure Your Child Bounty—Never Let One Slip Away!"}</p>
                                        </div>

                            //         }
                            //     } else {
                            //         html! {
                            //             <div class="flex flex-col flex-1 mt-28 items-center">
                            //                 <OnboardingSteps runtime={current_runtime.clone()} />
                            //             </div>
                            //         }
                            //     }
                            // }

                            <div class="hidden sm:flex">
                                <Footer runtime={current_runtime.clone()} disabled={!state.network.is_active()} onchange={&onchange_network} />
                            </div>
                        </div>

                        <div class="sm:col-span-2">

                            <div class="flex-auto sm:h-screen w-full overflow-hidden sm:overflow-auto">

                                <div class="flex flex-col items-center my-4 mt-20 md:mt-32 px-4 sm:px-0">

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

                        <div class="sm:hidden">
                            <Footer runtime={current_runtime.clone()} disabled={!state.network.is_active()} onchange={&onchange_network} />
                        </div>

                    </div>

                    <ClaimModal />
                    <AddAccountModal />
                </ContextProvider<StateContext>>
            </div>
        </>
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <ReactorProvider<BlockSubscription> path="/network_subscription_worker.js">
            <OneshotProvider<StorageQueries> path="/network_storage_worker.js">
                <Main />
            </OneshotProvider<StorageQueries>>
        </ReactorProvider<BlockSubscription>>
    }
}
