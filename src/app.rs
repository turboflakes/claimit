use crate::providers::network::NetworkStatus;
use crate::router::Query;
use crate::runtimes::support::SupportedRelayRuntime;
use crate::state::{account_key, Action, State, StateContext};
use crate::workers::{
    network_storage::{Query as StorageQuery, Response as StorageResponse, StorageQueries},
    network_subscription::{
        BlockSubscription, Input as SubscriptionInput, Output as SubscriptionOutput,
    },
};
use crate::{
    components::{buttons::NetworkSubscriber, inputs::AccountInput, items::AccountItem},
    providers::network::NetworkState,
};
use gloo::storage::{LocalStorage, Storage};
use log::{debug, info};
use yew::{
    classes, function_component, html,
    platform::spawn_local,
    prelude::{use_effect_with, use_reducer, UseReducerHandle},
    use_callback, Callback, ContextProvider, Html,
};
use yew_agent::oneshot::{use_oneshot_runner, OneshotProvider};
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

    let storage_task = use_oneshot_runner::<StorageQueries>();

    // State
    let state = use_reducer(|| {
        debug!("state initiated!");
        State {
            accounts: LocalStorage::get(account_key(current_runtime.clone()))
                .unwrap_or_else(|_| vec![]),
            network: NetworkState::new(current_runtime.clone()),
            child_bounties_raw: None,
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
                SubscriptionOutput::Active => {
                    state_cloned.dispatch(Action::ChangeNetworkStatus(NetworkStatus::Active));

                    // Query all child bounties
                    let storage_task = storage_task.clone();
                    let runtime = current_runtime.clone();

                    let state_cloned = state_cloned.clone();
                    spawn_local(async move {
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
                SubscriptionOutput::BlockNumber(n) => {
                    state_cloned.dispatch(Action::UpdateBlockNumber(n));
                }
                SubscriptionOutput::Err => {
                    state_cloned.dispatch(Action::ChangeNetworkStatus(NetworkStatus::Inactive));
                }
            },
            ReactorEvent::Finished => info!("__finished:"),
        });

    // Start subscription
    subscription_bridge.send(SubscriptionInput::StartSubscription(state.network.runtime));

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
    let ontoggle = make_callback(&state, Action::Toggle);
    // let onchange_network = make_callback(&state, Action::ChangeNetwork);
    let onchange_network = use_callback(
        (state.clone(), subscription_bridge.clone()),
        |runtime, (state, subscription_bridge)| {
            subscription_bridge.send(SubscriptionInput::Finish);
            subscription_bridge.reset();
            state.dispatch(Action::ChangeNetwork(runtime));
        },
    );

    let hidden_class = if state.accounts.is_empty() {
        "hidden"
    } else {
        ""
    };

    // Html
    html! {

        <ContextProvider<StateContext> context={state.clone()}>
            <AccountInput placeholder="Which beneficiary account are you looking for?" onenter={&onadd} />
            <section class={classes!("main", hidden_class)}>
                <ul class="accounts__list">
                    { for state.accounts.iter().cloned().map(|account|
                        html! {
                            <AccountItem {account}
                                onremove={&onremove}
                                ontoggle={&ontoggle}
                            />
                    }) }
                </ul>
            </section>

            <span>{state.network.runtime.to_string()}</span>
            
            <NetworkSubscriber selected={current_runtime} onchange={onchange_network} />

            <div>{state.network.status.to_string()}</div>

            {
                if state.network.finalized_block_number.is_some() {
                    html! {<div>{state.network.finalized_block_number.unwrap().to_string()}</div>}
                } else { html! {} }
            }
        </ContextProvider<StateContext>>

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
