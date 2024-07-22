use crate::providers::network::NetworkStatus;
use crate::router::Query;
use crate::runtimes::support::SupportedRelayRuntime;
use crate::state::{account_key, Action, State, StateContext};
use crate::types::{accounts::Account, child_bounties::Filter};
use crate::workers::{
    network_storage::{Query as StorageQuery, Response as StorageResponse, StorageQueries},
    network_subscription::{
        BlockSubscription, Input as SubscriptionInput, Output as SubscriptionOutput,
    },
};
use crate::{
    components::{
        inputs::AccountInput,
        items::{AccountItem, ChildBountyItem, FilterItem},
        nav::{Footer, Navbar},
    },
    providers::network::NetworkState,
};
use gloo::storage::{LocalStorage, Storage};
use log::debug;
use std::str::FromStr;
use strum::IntoEnumIterator;
use subxt::config::substrate::AccountId32;
use yew::{
    classes, function_component, html,
    platform::spawn_local,
    prelude::{use_effect_with, use_reducer, UseReducerHandle},
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
            .map(|a| AccountId32::from_str(&a.ss58).unwrap())
            .collect::<Vec<AccountId32>>();

        let filter = match following.len() {
            0 => Filter::All,
            _ => Filter::Following(following),
        };

        State {
            accounts,
            network: NetworkState::new(current_runtime.clone()),
            child_bounties_raw: None,
            filter,
        }
    });

    // Effect
    use_effect_with(state.clone(), |_state| {
        debug!("state changed!");
    });

    // Handle subscription responses over bridge
    let state_cloned = state.clone();
    let subscription_bridge: UseReactorBridgeHandle<BlockSubscription> =
        use_reactor_bridge(move |response| match response {
            ReactorEvent::Output(output) => match output {
                SubscriptionOutput::Active(_sub_id) => {
                    state_cloned.dispatch(Action::ChangeNetworkStatus(NetworkStatus::Active));

                    // Query all child bounties
                    let storage_task = storage_task.clone();
                    let runtime = current_runtime.clone();

                    let state_cloned = state_cloned.clone();
                    spawn_local(async move {
                        state_cloned.dispatch(Action::AddFetch);
                        let response = storage_task
                            .run(StorageQuery::FetchChildBounties(runtime))
                            .await;
                        match response {
                            StorageResponse::ChildBounties(data) => {
                                state_cloned.dispatch(Action::UpdateChildBountiesRaw(data))
                            }
                        };
                    });
                }
                SubscriptionOutput::BlockNumber(sub_id, block_number) => {
                    if state_cloned.network.subscription_id == sub_id {
                        state_cloned.dispatch(Action::UpdateBlockNumber(block_number));
                    }
                }
                SubscriptionOutput::Err(_) => {
                    state_cloned.dispatch(Action::ChangeNetworkStatus(NetworkStatus::Inactive));
                }
            },
            ReactorEvent::Finished => debug!("subscription finished"),
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
    let onremove = make_callback(&state, Action::Remove);
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
    let onset_filter = make_callback(&state, Action::SetFilter);

    let _hidden_class = if state.accounts.is_empty() {
        "hidden"
    } else {
        ""
    };

    // Html
    html! {
        <>
            <div class={classes!("flex", "min-h-screen", "overflow-y-none", current_runtime.class())}>
                <ContextProvider<StateContext> context={state.clone()}>

                    <Navbar runtime={current_runtime.clone()} />

                    <div class="flex w-full items-center justify-center">

                        <div class=" max-w-[768px] flex flex-col flex-1">

                            // header
                            <div class="flex flex-col w-full items-center justify-center my-4">
                                <img class="mb-8 max-w-[256px]" src="/images/claimeer_logo_black.svg" alt="Claimeer logo" />
                                <p class="text-base text-light text-gray-900">{"Everything you need to stay on top and claim your favourite Child Bounties."}</p>
                            </div>

                            //  search
                            <AccountInput placeholder="Enter the child bounty beneficiary account you wish to keep track of..." onenter={&onadd} />

                            // child bounties
                            <div class="md:flex">

                                {
                                    if state.accounts.len() > 0 {
                                        html! {
                                            <div>
                                                <h4 class="text-sm text-gray-500 dark:text-gray-100 mb-2">{"Favourite accounts"}</h4>
                                                <ul class="flex-column space-y space-y-4 text-sm font-medium text-gray-500 dark:text-gray-400 md:me-4 mb-4 md:mb-0">
                                                    { for state.accounts.iter().cloned().map(|account|
                                                        html! {
                                                            <AccountItem {account}  onunfollow={onremove.clone()} />
                                                    }) }
                                                </ul>
                                            </div>

                                        }
                                    } else { html! {} }
                                }

                                <div class="p-6 bg-gray-50 text-medium text-gray-500 dark:text-gray-400 dark:bg-gray-800 rounded-lg w-full">

                                    <div class="flex mb-4 justify-between items-center ">
                                        <h3 class="text-lg font-bold text-gray-900 dark:text-gray-100 mb-2">{"Child Bounties"}</h3>
                                        <ul class="tab tab__filters">
                                            { for Filter::iter().map(|filter| {
                                                html! {
                                                    <FilterItem filter={filter.clone()}
                                                        selected={state.filter.to_string() == filter.to_string()}
                                                        onclick={&onset_filter}
                                                    />
                                                }
                                            }) }
                                        </ul>
                                    </div>

                                    {
                                        if state.child_bounties_raw.is_some() {
                                            html! {
                                                <>
                                                    <ul class="flex-column space-y space-y-4 text-sm font-medium text-gray-500 dark:text-gray-400 overflow-y-scroll h-96">
                                                        { for state.child_bounties_raw.clone().unwrap().into_iter().filter(|(_, cb)| state.filter.check(cb)).map(|(_, cb)|
                                                            html! {
                                                                <ChildBountyItem child_bounty={cb} />
                                                            })
                                                        }
                                                    </ul>
                                                </>
                                            }
                                        } else { html! { <p>{"No child bounties available!"}</p> }}
                                    }

                                </div>
                            </div>
                        </div>
                    </div>
                    <Footer runtime={current_runtime.clone()} disabled={!state.network.is_active()} onchange={&onchange_network} />
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
