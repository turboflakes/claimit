use crate::components::spinners::Spinner;
use crate::router::{Query, Routes};
use crate::state::Action;
use crate::state::StateContext;
use claimeer_common::runtimes::support::SupportedRelayRuntime;
use claimeer_common::types::child_bounties::Id;
use std::str::FromStr;
use subxt::config::substrate::AccountId32;
use yew::{
    classes, function_component, html, use_context, AttrValue, Callback, Children, Html, Properties,
};
use yew_router::prelude::use_navigator;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: AttrValue,
    pub label: AttrValue,
    pub children: Children,
    pub onclick: Callback<()>,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let onclick = props.onclick.reform(move |_| ());

    html! {
        <button class={classes!("btn", props.class.clone())} {onclick} disabled={props.disabled.clone()}>
            {props.children.clone()}<span class="label">{format!("{}", props.label.to_string())}</span>
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct AnotherButtonProps {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: AttrValue,
    pub label: AttrValue,
    pub children: Children,
    pub onclick: Callback<()>,
}

#[function_component(AnotherButton)]
pub fn another_button(props: &AnotherButtonProps) -> Html {
    let onclick = props.onclick.reform(move |_| ());

    html! {
        <button class={classes!("btn", props.class.clone())} {onclick} disabled={props.disabled.clone()}>
            {props.children.clone()}<span class="label">{format!("{}", props.label.to_string())}</span>
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct NetworkButtonProps {
    pub chain: SupportedRelayRuntime,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    #[prop_or_default]
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
        <button class={classes!("btn", "btn__link", optional_class)} {onclick} disabled={props.disabled.clone()}>
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
                },
            }}
        </>
    }
}

#[function_component(ClaimButton)]
pub fn claim_button() -> Html {
    let state = use_context::<StateContext>().unwrap();

    let cbs: Vec<Id> = if let Some(block_number) = state.network.finalized_block_number {
        if let Some(child_bounties_raw) = &state.child_bounties_raw {
            let accounts = state
                .accounts
                .iter()
                .map(|a| AccountId32::from_str(&a.address).unwrap())
                .collect::<Vec<AccountId32>>();
            let cbs = child_bounties_raw
                .into_iter()
                .filter(|(_, cb)| {
                    cb.is_claimable(block_number) && accounts.contains(&cb.beneficiary)
                })
                .map(|(_, cb)| cb.id.clone())
                .collect::<Vec<Id>>();
            cbs
        } else {
            Vec::new()
        }
    } else {
        Vec::new()
    };

    let visibility = if cbs.len() > 0 {
        Some("inline-flex")
    } else {
        Some("hidden")
    };

    let onclick = {
        let cbs = cbs.clone();
        Callback::from(move |_| {
            state.dispatch(Action::StartClaim(cbs.clone()));
        })
    };

    html! {
        <button type="button" class={classes!("btn__claim", visibility)} {onclick} >
            {"Claim"}
            <span class="inline-flex items-center justify-center w-5 h-5 ms-2 text-xs font-semibold text-gray-100 bg-gray-900 rounded-full">
            {cbs.len()}
            </span>
        </button>
    }
}

#[function_component(SignButton)]
pub fn sign_button() -> Html {
    let state = use_context::<StateContext>().unwrap();
    let extension = state.extension.clone();
    let claim = state.claim.clone();

    if let Some(claim) = claim {
        let onclick = {
            let state = state.clone();
            let claim = state.claim.clone();
            Callback::from(move |_| {
                if let Some(claim) = claim.clone() {
                    state.dispatch(Action::SignClaim(claim));
                }
            })
        };

        let label = if claim.is_signing_or_submitting() {
            html! {
                <span class="inline-flex items-center"><Spinner is_visible={true} />{claim.status.to_string()}</span>
            }
        } else {
            html! { "Sign and Submit" }
        };

        html! {
            <button type="button" class="btn btn__primary" {onclick} disabled={!extension.is_ready() || claim.is_signing_or_submitting()} >{label}</button>
        }
    } else {
        html! {}
    }
}
