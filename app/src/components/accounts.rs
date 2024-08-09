use crate::components::{
    buttons::AddAccountButton,
    items::{AccountItem, AccountItemSmall},
};
use crate::state::{Action, StateContext};
use claimeer_common::runtimes::support::SupportedRelayRuntime;
use yew::{function_component, html, use_context, Callback, Html, Properties};

#[function_component(AccountsCardSmall)]
pub fn accounts_card_small() -> Html {
    let state = use_context::<StateContext>().unwrap();

    let onunfollow = {
        let state = state.clone();
        Callback::from(move |id| {
            state.dispatch(Action::RemoveAccountId(id));
        })
    };

    if state.accounts.len() > 0 {
        html! {
            <div class="mb-4">
                <ul class="flex flex-wrap items-center mx-2 text-xs font-medium text-gray-500 dark:text-gray-400">
                    { for state.accounts.iter().cloned().map(|account|
                        html! {
                            <AccountItemSmall {account}  onunfollow={&onunfollow} />
                    }) }
                </ul>
            </div>
        }
    } else {
        html! {}
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct AccountsCardProps {
    pub runtime: SupportedRelayRuntime,
}

#[function_component(AccountsCard)]
pub fn accounts_card(props: &AccountsCardProps) -> Html {
    let state = use_context::<StateContext>().unwrap();

    let onunfollow = {
        let state = state.clone();
        Callback::from(move |id| {
            state.dispatch(Action::RemoveAccountId(id));
        })
    };

    if state.accounts.len() > 0 {
        html! {
            <div class="mb-4">
                <ul class="flex flex-nowrap items-center mx-2 text-xs font-medium text-gray-500 dark:text-gray-400">
                    { for state.accounts.iter().cloned().map(|account|
                        html! {
                            <AccountItem {account} runtime={props.runtime.clone()} onunfollow={&onunfollow} />
                    }) }

                    <AddAccountButton />

                </ul>
            </div>
        }
    } else {
        html! {}
    }
}
