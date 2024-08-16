use crate::components::icons::Identicon;
use crate::state::{Action, StateContext};
use claimeer_common::runtimes::utils::compact;
use subxt::config::substrate::AccountId32;
use yew::{classes, function_component, html, use_context, AttrValue, Callback, Html, Properties};

#[derive(PartialEq, Properties, Clone)]
pub struct AccountChipProps {
    #[prop_or_default]
    pub class: AttrValue,
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
            state.dispatch(Action::AddAccount(account.clone()));
        })
    };

    html! {
        <span class={classes!("account__chip", props.class.clone())}>
            <div class="inline-flex items-center">
                <Identicon address={props.account.to_string()} size={24} class="me-2" />
                <span class="me-2">{compact(&props.account.clone())}</span>
            </div>
            {
                if !is_already_following {
                    html! {
                        <button type="button" class={classes!("btn", "btn__icon", state.network.runtime.class())} aria-label="Follow Account"
                            {onclick} >
                            <svg class="w-4 h-4 text-white dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14m-7 7V5"/>
                            </svg>

                            <span class="sr-only">{"Follow"}</span>
                        </button>
                    }
                } else { html! {} }
            }
        </span>

    }
}
