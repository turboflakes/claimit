use crate::router::{Query, Routes};
use crate::runtimes::support::SupportedRelayRuntime;
use yew::{classes, function_component, html, AttrValue, Callback, Children, Html, Properties};
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
    pub class: Option<AttrValue>,
    pub children: Children,
    pub onclick: Callback<SupportedRelayRuntime>,
}

#[function_component(NetworkButton)]
pub fn network_button(props: &NetworkButtonProps) -> Html {
    let optional_class = props.class.clone();
    let chain = props.chain.clone();

    let onclick = props.onclick.reform(move |_| chain);

    html! {
        <button class={classes!("btn__icon", optional_class)} {onclick} >
            {props.children.clone()}
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct NetworkSubscriberProps {
    pub selected: SupportedRelayRuntime,
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

    // let visible_class = if self.network_state.is_active() {
    //     Some("visible")
    // } else {
    //     Some("hidden")
    // };

    let visible_class = Some("visible");

    html! {
        <>
            { match selected {
                SupportedRelayRuntime::Polkadot => html! {
                    <NetworkButton chain={SupportedRelayRuntime::Kusama} class={visible_class} onclick={onclick.clone()} >
                        <img class="icon__img" src="/images/kusama_icon.svg" alt="kusama logo" />
                    </NetworkButton>
                },
                SupportedRelayRuntime::Kusama => html! {
                    <NetworkButton chain={SupportedRelayRuntime::Polkadot} class={visible_class} onclick={onclick.clone()} >
                        <img class="icon__img" src="/images/polkadot_icon_white.svg" alt="polkadot logo" />
                    </NetworkButton>
                }
            }}
        </>
    }
}
