use crate::components::{items::{AccountItem, AccountItemSmall}, buttons::AddAccountButton};
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
            <div class="relative px-4 max-w-[375px] md:max-w-[620px] overflow-auto">
                <ul class="flex flex-nowrap gap-4 items-center py-4 text-xs font-medium text-gray-500 dark:text-gray-400">
                    { for state.accounts.iter().cloned().map(|account|
                        html! {
                            <AccountItem {account} runtime={props.runtime.clone()} onunfollow={&onunfollow} />
                    }) }

                    <li class="account__item">
                        <AddAccountButton />
                    </li>

                </ul>
            </div>
        }
    } else {
        html! {}
    }
}
