use crate::components::spinners::Spinner;
use crate::router::{Query, Routes};
use crate::state::Action;
use crate::state::StateContext;
use claimit_common::runtimes::support::SupportedRelayRuntime;
use claimit_common::types::{child_bounties::ChildBountiesIds, layout::BalanceMode};
use std::str::FromStr;
use subxt::config::substrate::AccountId32;
use yew::{
    classes, function_component, html, use_context, AttrValue, Callback, Children, Html, Properties,
};
use yew_router::prelude::{use_location, use_navigator};

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
pub struct ExtensionButtonProps {
    pub name: AttrValue,
    #[prop_or_default]
    pub class: Option<AttrValue>,
    pub label: AttrValue,
    #[prop_or_default]
    pub disabled: bool,
    pub children: Children,
    pub onclick: Callback<AttrValue>,
}

#[function_component(ExtensionButton)]
pub fn extension_button(props: &ExtensionButtonProps) -> Html {
    let optional_class = props.class.clone();
    let name = props.name.clone();

    let onclick = props.onclick.reform(move |_| name.clone());

    html! {
        <button class={classes!("btn", optional_class)} {onclick} disabled={props.disabled.clone()}>
            <span class="inline-flex items-center gap-2">
                {props.children.clone()}
                <div class="label">
                    <span>{format!("{}", props.label.to_string())}</span>
                    {
                        if props.disabled {
                            html! { <span class="text-gray-600 text-xs font-light">{"Not installed"}</span> }
                        } else {
                            html! {}
                        }
                    }
                </div>
            </span>
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
    let location = use_location().unwrap();
    let light_client_param = location.query::<Query>().map(|q| q.lc).unwrap_or_default();

    let onclick = props.onchange.reform({
        let lc = light_client_param.clone();
        move |chain| {
            navigator
                .push_with_query(&Routes::Index, &Query { chain, lc })
                .unwrap();
            chain
        }
    });

    html! {
        <>
            { match selected {
                SupportedRelayRuntime::Polkadot => html! {
                    <NetworkButton chain={SupportedRelayRuntime::Kusama} disabled={props.disabled.clone()} onclick={onclick.clone()} >
                        <img class="h-8" src="/images/kusama_icon.svg" alt="kusama logo" />
                        <span class="font-bold tracking-wide">{"Switch to Kusama"}</span>
                    </NetworkButton>
                },
                SupportedRelayRuntime::Kusama => html! {
                    <NetworkButton chain={SupportedRelayRuntime::Polkadot} disabled={props.disabled.clone()} onclick={onclick.clone()} >
                        <img class="h-8" src="/images/polkadot_icon.svg" alt="polkadot logo" />
                        <span class="font-bold tracking-wide">{"Switch to Polkadot"}</span>
                    </NetworkButton>
                },
                SupportedRelayRuntime::Paseo => html! {
                    <NetworkButton chain={SupportedRelayRuntime::Polkadot} disabled={props.disabled.clone()} onclick={onclick.clone()} >
                        <img class="h-8" src="/images/polkadot_icon.svg" alt="polkadot logo" />
                        <span class="font-bold tracking-wide">{"Switch to Polkadot"}</span>
                    </NetworkButton>
                },
            }}
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct NetworkProviderIconButtonProps {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: AttrValue,
    pub onclick: Callback<bool>,
}

#[function_component(NetworkProviderIconButton)]
pub fn network_provider_icon_button(props: &NetworkProviderIconButtonProps) -> Html {
    let state = use_context::<StateContext>().unwrap();
    let navigator = use_navigator().unwrap();
    let location = use_location().unwrap();
    let chain_param = location
        .query::<Query>()
        .map(|q| q.chain)
        .unwrap_or_default();
    let light_client_param = location.query::<Query>().map(|q| q.lc).unwrap_or(true);

    let onclick = props.onclick.reform({
        let chain = chain_param.clone();
        let lc = !light_client_param;
        move |_| {
            navigator
                .push_with_query(&Routes::Index, &Query { chain, lc })
                .unwrap();
            lc
        }
    });

    html! {
        <button class={classes!("btn", "btn__icon", "btn__gray", props.class.clone())} {onclick} disabled={props.disabled.clone()} title={state.network.provider_description()} >
           {
                if state.network.is_ligh_client() {
                    html! {
                        <svg class={classes!("w-4", "h-4", state.network.status.text_class())} aria-hidden="true" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
                            <path d="M278.5 215.6L23 471c-9.4 9.4-9.4 24.6 0 33.9s24.6 9.4 33.9 0l74.8-74.8c7.4 4.6 15.3 8.2 23.8 10.5C200.3 452.8 270 454.5 338 409.4c12.2-8.1 5.8-25.4-8.8-25.4l-16.1 0c-5.1 0-9.2-4.1-9.2-9.2c0-4.1 2.7-7.6 6.5-8.8l97.7-29.3c3.4-1 6.4-3.1 8.4-6.1c4.4-6.4 8.6-12.9 12.6-19.6c6.2-10.3-1.5-23-13.5-23l-38.6 0c-5.1 0-9.2-4.1-9.2-9.2c0-4.1 2.7-7.6 6.5-8.8l80.9-24.3c4.6-1.4 8.4-4.8 10.2-9.3C494.5 163 507.8 86.1 511.9 36.8c.8-9.9-3-19.6-10-26.6s-16.7-10.8-26.6-10C391.5 7 228.5 40.5 137.4 131.6C57.3 211.7 56.7 302.3 71.3 356.4c2.1 7.9 12 9.6 17.8 3.8L253.6 195.8c6.2-6.2 16.4-6.2 22.6 0c5.4 5.4 6.1 13.6 2.2 19.8z" fill="currentColor" fill-rule="nonzero"/>
                        </svg>
                    }
                } else {
                    html! {
                        <svg class={classes!("w-4", "h-4", state.network.status.text_class())} aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                            <path fill-rule="evenodd" d="M5 5a2 2 0 0 0-2 2v3a1 1 0 0 0 1 1h16a1 1 0 0 0 1-1V7a2 2 0 0 0-2-2H5Zm9 2a1 1 0 1 0 0 2h.01a1 1 0 1 0 0-2H14Zm3 0a1 1 0 1 0 0 2h.01a1 1 0 1 0 0-2H17ZM3 17v-3a1 1 0 0 1 1-1h16a1 1 0 0 1 1 1v3a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2Zm11-2a1 1 0 1 0 0 2h.01a1 1 0 1 0 0-2H14Zm3 0a1 1 0 1 0 0 2h.01a1 1 0 1 0 0-2H17Z" clip-rule="evenodd"/>
                        </svg>
                    }
                }
           }
        </button>
    }
}

#[function_component(ClaimButton)]
pub fn claim_button() -> Html {
    let state = use_context::<StateContext>().unwrap();

    let cbs: ChildBountiesIds = if let Some(block_number) = state.network.finalized_block_number {
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
                .map(|(_, cb)| (cb.parent_id.clone(), cb.id.clone()))
                .collect::<ChildBountiesIds>();
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
        let state = state.clone();
        let cbs = cbs.clone();
        Callback::from(move |_| {
            state.dispatch(Action::StartClaim(cbs.clone()));
        })
    };

    html! {
        <button type="button" class={classes!("group", "btn__claim", state.network.runtime.class(), visibility)} {onclick} >
            <svg class="w-5 h-5 text-inherit dark:text-white me-2" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                <path fill-rule="evenodd" d="M20.337 3.664c.213.212.354.486.404.782.294 1.711.657 5.195-.906 6.76-1.77 1.768-8.485 5.517-10.611 6.683a.987.987 0 0 1-1.176-.173l-.882-.88-.877-.884a.988.988 0 0 1-.173-1.177c1.165-2.126 4.913-8.841 6.682-10.611 1.562-1.563 5.046-1.198 6.757-.904.296.05.57.191.782.404ZM5.407 7.576l4-.341-2.69 4.48-2.857-.334a.996.996 0 0 1-.565-1.694l2.112-2.111Zm11.357 7.02-.34 4-2.111 2.113a.996.996 0 0 1-1.69-.565l-.422-2.807 4.563-2.74Zm.84-6.21a1.99 1.99 0 1 1-3.98 0 1.99 1.99 0 0 1 3.98 0Z" clip-rule="evenodd"/>
            </svg>
            {"Claim"}
            <span class={classes!("counter", state.network.runtime.class())}>
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
            Callback::from(move |_| {
                state.dispatch(Action::PreparePayload);
            })
        };

        let label = if claim.is_signing_or_submitting() {
            html! {
                <span class="inline-flex items-center"><Spinner class="me-2" is_visible={true} />{claim.status.to_string()}</span>
            }
        } else {
            html! { "Sign and Submit" }
        };

        html! {
            <button type="button" class={classes!("btn", "btn__primary", state.network.runtime.class())} {onclick} disabled={!extension.is_ready() || claim.is_signing_or_submitting()} >{label}</button>
        }
    } else {
        html! {}
    }
}

#[derive(Properties, PartialEq)]
pub struct AddAccountButtonProps {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: AttrValue,
    // pub onclick: Callback<()>,
}

#[function_component(AddAccountButton)]
pub fn add_account_button(props: &AddAccountButtonProps) -> Html {
    let state = use_context::<StateContext>().unwrap();

    let onclick = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(Action::ToggleLayoutAddAccountModal);
        })
    };

    html! {
        <button class={classes!("btn__add_account", props.class.clone())} {onclick}>
            <div class="inline-flex items-center">
                <svg class="w-12 h-12 text-gray-900 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                    <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14m-7 7V5"/>
                </svg>
                <p class="ms-2 text-gray-900 text-3xl">{"Add"}</p>
            </div>

            <p class="text-xs text-left text-gray-500">{"Add a child bounty beneficiary account you want to follow and claim!"}</p>
        </button>
    }
}

#[function_component(BalanceButtonGroup)]
pub fn balance_button_group() -> Html {
    let state = use_context::<StateContext>().unwrap();

    let onclick = {
        let state = state.clone();
        Callback::from(move |e| {
            state.dispatch(Action::ChangeBalanceMode(e));
        })
    };

    html! {
        <div class="flex flex-wrap justify-end max-w-32 gap-2 sm:inline-flex sm:flex-nowrap sm:max-w-full">
            <TotalBalanceIconButton onclick={&onclick} disabled={state.layout.is_total_balance_mode()} />
            <TotalAwardedIconButton onclick={&onclick} disabled={state.layout.is_total_awarded_mode() || state.network.is_busy()} />
            <TotalPendingIconButton onclick={&onclick} disabled={state.layout.is_total_pending_mode() || state.network.is_busy()} />
            <TotalClaimableIconButton onclick={&onclick} disabled={state.layout.is_total_claimable_mode() || state.network.is_busy()} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TotalBalanceIconButtonProps {
    #[prop_or_default]
    pub disabled: bool,
    pub onclick: Callback<BalanceMode>,
}

#[function_component(TotalBalanceIconButton)]
pub fn total_balance_icon_button(props: &TotalBalanceIconButtonProps) -> Html {
    let onclick = props.onclick.reform(move |_| BalanceMode::TotalBalance);

    html! {
        <button class={classes!("btn", "btn__icon", "btn__white")} {onclick} disabled={props.disabled.clone()}>
            <svg class="w-6 h-6 text-inherent dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                <path fill-rule="evenodd" d="M12 14a3 3 0 0 1 3-3h4a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2h-4a3 3 0 0 1-3-3Zm3-1a1 1 0 1 0 0 2h4v-2h-4Z" clip-rule="evenodd"/>
                <path fill-rule="evenodd" d="M12.293 3.293a1 1 0 0 1 1.414 0L16.414 6h-2.828l-1.293-1.293a1 1 0 0 1 0-1.414ZM12.414 6 9.707 3.293a1 1 0 0 0-1.414 0L5.586 6h6.828ZM4.586 7l-.056.055A2 2 0 0 0 3 9v10a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2h-4a5 5 0 0 1 0-10h4a2 2 0 0 0-1.53-1.945L17.414 7H4.586Z" clip-rule="evenodd"/>
            </svg>
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct TotalAwardedIconButtonProps {
    #[prop_or_default]
    pub disabled: bool,
    pub onclick: Callback<BalanceMode>,
}

#[function_component(TotalAwardedIconButton)]
pub fn total_awarded_icon_button(props: &TotalAwardedIconButtonProps) -> Html {
    let onclick = props.onclick.reform(move |_| BalanceMode::TotalAwarded);

    html! {
        <button class={classes!("btn", "btn__icon", "btn__white")} {onclick} disabled={props.disabled.clone()}>
            <svg class="w-6 h-6 text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                <path d="M11 9a1 1 0 1 1 2 0 1 1 0 0 1-2 0Z"/>
                <path fill-rule="evenodd" d="M9.896 3.051a2.681 2.681 0 0 1 4.208 0c.147.186.38.282.615.255a2.681 2.681 0 0 1 2.976 2.975.681.681 0 0 0 .254.615 2.681 2.681 0 0 1 0 4.208.682.682 0 0 0-.254.615 2.681 2.681 0 0 1-2.976 2.976.681.681 0 0 0-.615.254 2.682 2.682 0 0 1-4.208 0 .681.681 0 0 0-.614-.255 2.681 2.681 0 0 1-2.976-2.975.681.681 0 0 0-.255-.615 2.681 2.681 0 0 1 0-4.208.681.681 0 0 0 .255-.615 2.681 2.681 0 0 1 2.976-2.975.681.681 0 0 0 .614-.255ZM12 6a3 3 0 1 0 0 6 3 3 0 0 0 0-6Z" clip-rule="evenodd"/>
                <path d="M5.395 15.055 4.07 19a1 1 0 0 0 1.264 1.267l1.95-.65 1.144 1.707A1 1 0 0 0 10.2 21.1l1.12-3.18a4.641 4.641 0 0 1-2.515-1.208 4.667 4.667 0 0 1-3.411-1.656Zm7.269 2.867 1.12 3.177a1 1 0 0 0 1.773.224l1.144-1.707 1.95.65A1 1 0 0 0 19.915 19l-1.32-3.93a4.667 4.667 0 0 1-3.4 1.642 4.643 4.643 0 0 1-2.53 1.21Z"/>
            </svg>
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct TotalPendingIconButtonProps {
    #[prop_or_default]
    pub disabled: bool,
    pub onclick: Callback<BalanceMode>,
}

#[function_component(TotalPendingIconButton)]
pub fn total_pending_icon_button(props: &TotalPendingIconButtonProps) -> Html {
    let onclick = props.onclick.reform(move |_| BalanceMode::TotalPending);

    html! {
        <button class={classes!("btn", "btn__icon", "btn__white")} {onclick} disabled={props.disabled.clone()}>
            <svg class="w-6 h-6 text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                <path fill-rule="evenodd" d="M10 5a2 2 0 0 0-2 2v3h2.4A7.48 7.48 0 0 0 8 15.5a7.48 7.48 0 0 0 2.4 5.5H5a2 2 0 0 1-2-2v-7a2 2 0 0 1 2-2h1V7a4 4 0 1 1 8 0v1.15a7.446 7.446 0 0 0-1.943.685A.999.999 0 0 1 12 8.5V7a2 2 0 0 0-2-2Z" clip-rule="evenodd"/>
                <path fill-rule="evenodd" d="M10 15.5a5.5 5.5 0 1 1 11 0 5.5 5.5 0 0 1-11 0Zm6.5-1.5a1 1 0 1 0-2 0v1.5a1 1 0 0 0 .293.707l1 1a1 1 0 0 0 1.414-1.414l-.707-.707V14Z" clip-rule="evenodd"/>
            </svg>
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct TotalClaimableIconButtonProps {
    #[prop_or_default]
    pub disabled: bool,
    pub onclick: Callback<BalanceMode>,
}

#[function_component(TotalClaimableIconButton)]
pub fn total_claimable_icon_button(props: &TotalClaimableIconButtonProps) -> Html {
    let onclick = props.onclick.reform(move |_| BalanceMode::TotalClaimable);

    html! {
        <button class={classes!("btn", "btn__icon", "btn__white")} {onclick} disabled={props.disabled.clone()}>
            <svg class="w-6 h-6 text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                <path fill-rule="evenodd" d="M20.337 3.664c.213.212.354.486.404.782.294 1.711.657 5.195-.906 6.76-1.77 1.768-8.485 5.517-10.611 6.683a.987.987 0 0 1-1.176-.173l-.882-.88-.877-.884a.988.988 0 0 1-.173-1.177c1.165-2.126 4.913-8.841 6.682-10.611 1.562-1.563 5.046-1.198 6.757-.904.296.05.57.191.782.404ZM5.407 7.576l4-.341-2.69 4.48-2.857-.334a.996.996 0 0 1-.565-1.694l2.112-2.111Zm11.357 7.02-.34 4-2.111 2.113a.996.996 0 0 1-1.69-.565l-.422-2.807 4.563-2.74Zm.84-6.21a1.99 1.99 0 1 1-3.98 0 1.99 1.99 0 0 1 3.98 0Z" clip-rule="evenodd"/>
            </svg>
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct NextIconButtonProps {
    #[prop_or_default]
    pub disabled: bool,
    pub onclick: Callback<()>,
}

#[function_component(NextIconButton)]
pub fn next_icon_button(props: &NextIconButtonProps) -> Html {
    let onclick = props.onclick.reform(move |_| ());

    html! {
        <button class={classes!("btn", "btn__icon", "btn__transparent")} {onclick} disabled={props.disabled.clone()}>
            <svg class="w-6 h-6 text-inherent dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m9 5 7 7-7 7"/>
            </svg>
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct BackIconButtonProps {
    #[prop_or_default]
    pub disabled: bool,
    pub onclick: Callback<()>,
}

#[function_component(BackIconButton)]
pub fn back_icon_button(props: &BackIconButtonProps) -> Html {
    let onclick = props.onclick.reform(move |_| ());

    html! {
        <button class={classes!("btn", "btn__icon", "btn__transparent")} {onclick} disabled={props.disabled.clone()}>
            <svg class="w-6 h-6 text-inherent dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m15 19-7-7 7-7"/>
            </svg>
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct SubsquareIconLinkProps {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub id: AttrValue,
    pub runtime: SupportedRelayRuntime,
}

#[function_component(SubsquareIconLink)]
pub fn subsquare_icon_link(props: &SubsquareIconLinkProps) -> Html {
    html! {
        <a href={format!("https://{}.subsquare.io/treasury/child-bounties/{}", props.runtime.class(), props.id.clone())} target="_blank" rel="noopener noreferrer"
            class={classes!("btn", "btn__icon_sm", "btn__gray")} disabled={props.disabled.clone()}>
            <img class="w-3 h-3" src="/images/subsquare_icon.svg" alt="subsquare logo" />
        </a>
    }
}

#[derive(Properties, PartialEq)]
pub struct PolkassemblyIconLinkProps {
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub id: AttrValue,
    pub runtime: SupportedRelayRuntime,
}

#[function_component(PolkassemblyIconLink)]
pub fn polkassembly_icon_link(props: &PolkassemblyIconLinkProps) -> Html {
    html! {
        <a href={format!("https://{}.polkassembly.io/child_bounty/{}", props.runtime.class(), props.id.clone())} target="_blank" rel="noopener noreferrer"
            class={classes!("btn", "btn__icon_sm", "btn__gray")} disabled={props.disabled.clone()}>
            <img class="w-3 h-3" src="/images/polkassembly_icon.png" alt="polkassembly logo" />
        </a>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct BountyIdToggleProps {
    #[prop_or_default]
    pub class: AttrValue,
    pub id: u32,
    #[prop_or_default]
    pub selected: bool,
    pub onclick: Callback<u32>,
}

#[function_component(BountyIdToggle)]
pub fn bounty_id(props: &BountyIdToggleProps) -> Html {
    let id = props.id.clone();
    let onclick = props.onclick.reform(move |_| id);

    html! {
        <button class={classes!("btn", "btn__icon", "btn__white", props.selected.then(|| Some("selected")), props.class.clone())} {onclick}>
            <div class="inline-flex items-center">
                <span class="text-xs w-4 h-4">{props.id.clone()}</span>
            </div>
        </button>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct BountyAllToggleProps {
    #[prop_or_default]
    pub class: AttrValue,
    #[prop_or_default]
    pub selected: bool,
    pub onclick: Callback<()>,
}

#[function_component(BountyAllToggle)]
pub fn bounty_all(props: &BountyAllToggleProps) -> Html {
    let onclick = props.onclick.reform(move |_| ());

    html! {
        <button class={classes!("btn", "btn__icon", "btn__white", props.selected.then(|| Some("selected")), props.class.clone())} {onclick}>
            <div class="inline-flex items-center">
                <span class="text-xs w-20 h-4 ">{"All Bounties"}</span>
            </div>
        </button>
    }
}
