use crate::providers::network::NetworkStatus;
use crate::router::Query;
use crate::state::{account_key, signer_key, Action, State, StateContext};
use crate::workers::{
    network_storage::{Query as StorageQuery, Response as StorageResponse, StorageQueries},
    network_subscription::{
        BlockSubscription, Input as SubscriptionInput, Output as SubscriptionOutput,
    },
};
use crate::{
    components::{
        accounts::AccountsCard,
        child_bounties::ChildBountiesCard,
        inputs::AccountInput,
        items::{AccountItem, ChildBountyItem, FilterItem},
        modals::ClaimModal,
        nav::{Footer, Navbar},
    },
    providers::network::NetworkState,
};
use claimeer_common::runtimes::support::SupportedRelayRuntime;
use claimeer_common::types::{
    accounts::Account, child_bounties::Filter, extensions::ExtensionAccount,
    extensions::ExtensionState,
};
use gloo::storage::{LocalStorage, Storage};
use log::debug;
use log::info;
use std::str::FromStr;
use strum::IntoEnumIterator;
use subxt::config::substrate::AccountId32;
use yew::{
    classes, function_component, html,
    platform::spawn_local,
    prelude::{use_reducer, UseReducerHandle},
    use_callback, Callback, ContextProvider, Html,
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

        let following = accounts
            .iter()
            .map(|a| AccountId32::from_str(&a.address).unwrap())
            .collect::<Vec<AccountId32>>();

        let filter = match following.len() {
            0 => Filter::All,
            _ => Filter::Following(following),
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
                            _ => {}
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

    // Callbacks
    fn make_callback<E, F>(state: &UseReducerHandle<State>, f: F) -> Callback<E>
    where
        F: Fn(E) -> Action + 'static,
    {
        let state = state.clone();
        Callback::from(move |e: E| state.dispatch(f(e)))
    }

    let onadd = make_callback(&state, Action::Add);
    let _ontoggle = make_callback(&state, Action::Toggle);
    // let onchange_network = make_callback(&state, Action::ChangeNetwork);
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
            <div class={classes!("flex", "h-screen", current_runtime.class())}>
                <ContextProvider<StateContext> context={state.clone()}>

                    <Navbar runtime={current_runtime.clone()} />

                    <div class="flex flex-col w-full items-center justify-center">

                        <div class="mx-1 max-w-[610px] flex flex-col flex-1 justify-between h-full overflow-y-auto">


                            <div class="flex flex-col w-full items-center my-4 mt-16 md:mt-32">

                                <div class="flex flex-col w-full items-center">
                                    <img class="mb-8 max-w-[256px]" src="/images/claimeer_logo_black.svg" alt="Claimeer logo" />
                                    <p class="text-lg text-light text-gray-900">{"Everything you need to stay on top and claim your favourite Child Bounties."}</p>
                                </div>

                                <AccountInput placeholder="Enter the child bounty beneficiary account you wish to keep track of..." onenter={&onadd} />

                                <AccountsCard />

                                <ChildBountiesCard />

                            </div>

                            <Footer runtime={current_runtime.clone()} disabled={!state.network.is_active()} onchange={&onchange_network} />
                        </div>

                    </div>

                    <ClaimModal />
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
