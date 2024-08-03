use crate::components::items::AccountItem;
use crate::router::{Query, Routes};
use crate::state::{Action, StateContext};
use claimeer_common::runtimes::support::SupportedRelayRuntime;
use claimeer_common::types::{
    accounts::Account,
    child_bounties::{amount_human, Filter},
    extensions::ExtensionAccount,
    extensions::ExtensionState,
};
use std::str::FromStr;
use strum::IntoEnumIterator;
use subxt::config::substrate::AccountId32;
use yew::{
    classes, function_component, html, use_context, AttrValue, Callback, Children, Html, Properties,
};

#[function_component(AccountsCard)]
pub fn accounts_card() -> Html {
    let state = use_context::<StateContext>().unwrap();

    let onunfollow = {
        let state = state.clone();
        Callback::from(move |e| {
            state.dispatch(Action::Remove(e));
        })
    };

    if state.accounts.len() > 0 {
        html! {
            <div class="mb-4">
                <ul class="flex flex-wrap items-center mx-2 text-xs font-medium text-gray-500 dark:text-gray-400">
                    { for state.accounts.iter().cloned().map(|account|
                        html! {
                            <AccountItem {account}  onunfollow={&onunfollow} />
                    }) }
                </ul>
            </div>
        }
    } else {
        html! {}
    }
}
