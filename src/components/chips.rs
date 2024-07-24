use crate::components::icons::Identicon;
use crate::state::{Action, StateContext};
use crate::types::accounts::compact;
use subxt::config::substrate::AccountId32;
use yew::{function_component, html, use_context, Callback, Html, Properties};

#[derive(PartialEq, Properties, Clone)]
pub struct AccountChipProps {
    pub account: AccountId32,
}

#[function_component(AccountChip)]
pub fn account(props: &AccountChipProps) -> Html {
    let state = use_context::<StateContext>().unwrap();
    let is_already_following = state
        .accounts
        .iter()
        .any(|account| *account.address == props.account.to_string());

    let onclick = {
        let state = state.clone();
        let account = props.account.to_string();
        Callback::from(move |_| {
            state.dispatch(Action::Add(account.clone()));
        })
    };

    html! {
        <span class="inline-flex items-center me-2 text-sm min-h-9 text-gray-600 dark:text-gray-300">
            <Identicon address={props.account.to_string()} size={20} class="me-2" />
            {compact(&props.account.clone())}
            {
                if !is_already_following {
                    html! {
                        <button type="button" class="btn btn__icon" aria-label="Follow Account"
                            {onclick} >
                            <svg class="w-4 h-4 text-gray-600 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 12h4m-2 2v-4M4 18v-1a3 3 0 0 1 3-3h4a3 3 0 0 1 3 3v1a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1Zm8-10a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"/>
                            </svg>
                            <span class="sr-only">{"Follow"}</span>
                        </button>
                    }
                } else { html! {} }
            }
        </span>

    }
}
