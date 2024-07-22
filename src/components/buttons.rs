use crate::router::{Query, Routes};
use crate::runtimes::support::SupportedRelayRuntime;
use crate::state::StateContext;
use crate::types::child_bounties::ChildBounty;
use std::str::FromStr;
use subxt::config::substrate::AccountId32;
use yew::{
    classes, function_component, html, use_context, AttrValue, Callback, Children, Html, Properties,
};
use yew_router::prelude::use_navigator;

#[derive(Properties, PartialEq)]
pub struct ActionButtonProps {
    pub disable: bool,
    pub label: AttrValue,
    pub children: Children,
    pub onclick: Callback<()>,
}

#[function_component(ActionButton)]
pub fn button(props: &ActionButtonProps) -> Html {
    let onclick = props.onclick.reform(move |_| ());
    let disabled_class = if props.disable {
        Some("disabled")
    } else {
        None
    };

    html! {
        <div class="action__btn">
            <button class={classes!("btn__link", disabled_class)} {onclick}>
                {props.children.clone()}<span class="label">{format!("{}", props.label.to_string())}</span>
            </button>
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct NetworkButtonProps {
    pub chain: SupportedRelayRuntime,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    pub disabled: bool,
    pub children: Children,
    pub onclick: Callback<SupportedRelayRuntime>,
}

#[function_component(NetworkButton)]
pub fn network_button(props: &NetworkButtonProps) -> Html {
    let optional_class = props.class.clone();
    let chain = props.chain.clone();

    let onclick = props.onclick.reform(move |_| chain);

    html! {
        <button class={classes!("btn", optional_class)} {onclick} disabled={props.disabled.clone()}>
            {props.children.clone()}
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct NetworkSubscriberProps {
    pub selected: SupportedRelayRuntime,
    pub disabled: bool,
    pub onchange: Callback<SupportedRelayRuntime>,
}

#[function_component(NetworkSubscriber)]
pub fn network_subscriber(props: &NetworkSubscriberProps) -> Html {
    let selected = props.selected.clone();
    let navigator = use_navigator().unwrap();

    let onclick = props.onchange.reform(move |chain| {
        navigator
            .push_with_query(&Routes::Index, &Query { chain })
            .unwrap();

        chain
    });

    html! {
        <>
            { match selected {
                SupportedRelayRuntime::Polkadot => html! {
                    <NetworkButton chain={SupportedRelayRuntime::Kusama} disabled={props.disabled.clone()} onclick={onclick.clone()} >
                        <img class="h-8" src="/images/kusama_icon.svg" alt="kusama logo" />
                        <span>{"Switch to Kusama"}</span>
                    </NetworkButton>
                },
                SupportedRelayRuntime::Kusama => html! {
                    <NetworkButton chain={SupportedRelayRuntime::Polkadot} disabled={props.disabled.clone()} onclick={onclick.clone()} >
                        <img class="h-8" src="/images/polkadot_icon.svg" alt="polkadot logo" />
                        <span>{"Switch to Polkadot"}</span>
                    </NetworkButton>
                }
            }}
        </>
    }
}

#[function_component(ClaimButton)]
pub fn claim_button() -> Html {
    let state = use_context::<StateContext>().unwrap();

    let cbs: Vec<ChildBounty> = if let Some(block_number) = state.network.finalized_block_number {
        if let Some(child_bounties_raw) = &state.child_bounties_raw {
            let accounts = state
                .accounts
                .iter()
                .map(|a| AccountId32::from_str(&a.ss58).unwrap())
                .collect::<Vec<AccountId32>>();
            let cbs = child_bounties_raw
                .into_iter()
                .filter(|(_, cb)| {
                    cb.is_claimable(block_number) && accounts.contains(&cb.beneficiary)
                })
                .map(|(_, cb)| cb.clone())
                .collect::<Vec<ChildBounty>>();
            cbs
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    let visibility = if cbs.len() > 0 {
        Some("visible")
    } else {
        Some("hidden")
    };

    html! {
        <button type="button" class={classes!(visibility, "btn btn__claim".to_string())}>
            {"Claim"}
            <span class="inline-flex items-center justify-center w-5 h-5 ms-2 text-xs font-semibold text-gray-900 bg-white rounded-full">
            {cbs.len()}
            </span>
        </button>
    }
}
