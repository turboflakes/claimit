use crate::components::{
    accounts::AccountBalance,
    buttons::{PolkassemblyIconLink, SubsquareIconLink},
    chips::AccountChip,
    icons::Identicon,
};
use crate::state::{StateContext, Action};
use claimeer_common::runtimes::support::SupportedRelayRuntime;
use claimeer_common::types::{
    accounts::Account,
    child_bounties::{Filter, Id},
    extensions::ExtensionAccount,
};
use std::str::FromStr;
use subxt::config::substrate::AccountId32;
use yew::{
    classes, function_component, html, use_context, use_state, Callback, Classes, Html, MouseEvent,
    Properties,
};

#[derive(PartialEq, Properties, Clone)]
pub struct AccountItemSmallProps {
    pub account: Account,
    // pub ontoggle: Callback<usize>,
    pub onunfollow: Callback<u32>,
}

#[function_component(AccountItemSmall)]
pub fn account_item_small(props: &AccountItemSmallProps) -> Html {
    let is_dropdown_hidden = use_state(|| true);

    let id = props.account.id;

    let btn_dropdown_onclick = {
        let is_dropdown_hidden = is_dropdown_hidden.clone();
        Callback::from(move |_| {
            is_dropdown_hidden.set(!(*is_dropdown_hidden));
        })
    };

    let dropdown_onmouseleave = {
        let is_dropdown_hidden = is_dropdown_hidden.clone();

        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            is_dropdown_hidden.set(true);
        })
    };

    // let toggle_onclick = props.ontoggle.reform(move |_| id);
    let unfollow_onclick = props.onunfollow.reform(move |e: MouseEvent| {
        e.stop_propagation();
        id.try_into().unwrap()
    });

    html! {
        <li class="account__item_small">
            <div class="relative flex justify-between items-center px-3 py-2 rounded-lg text-gray-600 dark:text-gray-100 bg-gray-50 w-full dark:bg-gray-800">
                <div class="inline-flex items-center">
                    <Identicon address={props.account.address.clone()} size={24} class="me-2" />
                    {props.account.to_compact_string()}
                </div>
                <div class="inline-flex items-center">
                    <button type="button" class="btn btn__icon btn__white" onclick={btn_dropdown_onclick} >
                        <svg class="w-3 h-3 text-gray-600 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 4 15">
                            <path d="M3.5 1.5a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0Zm0 6.041a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0Zm0 5.959a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0Z"/>
                        </svg>
                        <span class="sr-only">{"Open dropdown"}</span>
                    </button>
                </div>
                <div class={classes!("menu_dropdown", (*is_dropdown_hidden).then(|| Some("hidden")))}
                    role="menu" aria-orientation="vertical" aria-labelledby="menu-button" tabindex="-1"
                    onmouseleave={dropdown_onmouseleave}>
                    <ul class="py-2 text-sm text-gray-700 dark:text-gray-200">
                        // <li>
                        //     <a href="" class="flex items-center px-4 py-2 hover:underline hover:underline-offset-4 dark:hover:text-white"
                        //         onclick={toggle_onclick} >
                        //         {"Disable"}
                        //     </a>
                        // </li>
                        // <hr/>
                        <li>
                            <div type="button" class="flex items-center px-4 py-2 hover:underline hover:underline-offset-4 dark:hover:text-white cursor-pointer"
                                onclick={unfollow_onclick}>
                                <svg class="w-4 h-4 text-gray-800 dark:text-white me-2" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                                    <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 12h4M4 18v-1a3 3 0 0 1 3-3h4a3 3 0 0 1 3 3v1a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1Zm8-10a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"/>
                                </svg>
                                {"Unfollow"}
                            </div>
                        </li>
                    </ul>
                </div>
                {
                    if props.account.child_bounty_ids.len() > 0 {
                        html! {
                            <div class="absolute -top-1 -left-1">
                                <span class="inline-flex items-center justify-center w-4 h-4 text-xs text-gray-100 bg-gray-900 rounded-full">
                                {props.account.child_bounty_ids.len()}
                                </span>
                            </div>
                        }
                    } else { html! {} }
                }
            </div>
        </li>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct AccountItemProps {
    pub account: Account,
    pub runtime: SupportedRelayRuntime,
    pub onunfollow: Callback<u32>,
}

#[function_component(AccountItem)]
pub fn account_item(props: &AccountItemProps) -> Html {
    let is_dropdown_hidden = use_state(|| true);

    let btn_dropdown_onclick = {
        let is_dropdown_hidden = is_dropdown_hidden.clone();
        Callback::from(move |_| {
            is_dropdown_hidden.set(!(*is_dropdown_hidden));
        })
    };

    let dropdown_onmouseleave = {
        let is_dropdown_hidden = is_dropdown_hidden.clone();

        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            is_dropdown_hidden.set(true);
        })
    };

    // let toggle_onclick = props.ontoggle.reform(move |_| id);
    let unfollow_onclick = props.onunfollow.reform({
        let id = props.account.id.clone();

        move |e: MouseEvent| {
            e.stop_propagation();
            id
        }
    });

    html! {
        <li class="account__item">
            <div class="w-64 h-48 flex flex-col justify-between p-4 rounded-lg text-gray-600 dark:text-gray-100 bg-gray-50 dark:bg-gray-800">
                <div class="inline-flex justify-between">
                    {
                        match props.runtime.clone() {
                            SupportedRelayRuntime::Polkadot => html! {
                                <div class="inline-flex items-center">
                                    <img class="h-12" src="/images/polkadot_icon.svg" alt="polkadot logo" />
                                    <p class="ms-2 text-xl font-semibold">{props.runtime.unit()}</p>
                                </div>
                            },
                            SupportedRelayRuntime::Kusama => html! {
                                <div class="inline-flex items-center">
                                    <img class="h-12" src="/images/kusama_icon.svg" alt="kusama logo" />
                                    <p class="ms-2 text-xl font-semibold">{props.runtime.unit()}</p>
                                </div>
                            },
                            SupportedRelayRuntime::Rococo => html! {
                                <div class="inline-flex items-center">
                                    <img class="h-12" src="/images/rococo_icon.svg" alt="rococo logo" />
                                    <p class="ms-2 text-xl font-semibold">{props.runtime.unit()}</p>
                                </div>
                            },
                        }
                    }
                    <div class="relative">
                        <div class="inline-flex items-center">
                            <button type="button" class="btn btn__icon btn__white" onclick={btn_dropdown_onclick} >
                                <svg class="w-3 h-3 text-gray-600 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 4 15">
                                    <path d="M3.5 1.5a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0Zm0 6.041a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0Zm0 5.959a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0Z"/>
                                </svg>
                                <span class="sr-only">{"Open dropdown"}</span>
                            </button>
                        </div>
                        <div class={classes!("menu_dropdown", (*is_dropdown_hidden).then(|| Some("hidden")))}
                            role="menu" aria-orientation="vertical" aria-labelledby="menu-button" tabindex="-1"
                            onmouseleave={dropdown_onmouseleave}>
                            <ul class="py-2 text-sm text-gray-700 dark:text-gray-200">
                                // <li>
                                //     <a href="" class="flex items-center px-4 py-2 hover:underline hover:underline-offset-4 dark:hover:text-white"
                                //         onclick={toggle_onclick} >
                                //         {"Disable"}
                                //     </a>
                                // </li>
                                // <hr/>
                                <li>
                                    <div type="button" class="flex items-center px-4 py-2 hover:underline hover:underline-offset-4 dark:hover:text-white cursor-pointer"
                                        onclick={unfollow_onclick}>
                                        <svg class="w-4 h-4 text-gray-800 dark:text-white me-2" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                                            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 12h4M4 18v-1a3 3 0 0 1 3-3h4a3 3 0 0 1 3 3v1a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1Zm8-10a3 3 0 1 1-6 0 3 3 0 0 1 6 0Z"/>
                                        </svg>
                                        {"Unfollow"}
                                    </div>
                                </li>
                            </ul>
                        </div>
                    </div>
                </div>
                <div class="flex flex-col">
                    <div class="inline-flex items-center mb-2">
                        <Identicon address={props.account.address.clone()} size={24} class="me-2" />
                        {props.account.to_compact_string()}
                    </div>

                    <AccountBalance runtime={props.runtime.clone()} account={props.account.clone()} />

                </div>
            </div>
        </li>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct ExtensionAccountItemProps {
    pub account: ExtensionAccount,
    #[prop_or_default]
    pub highlight: bool,
    pub onclick: Callback<ExtensionAccount>,
}

#[function_component(ExtensionAccountItem)]
pub fn extension_account_item(props: &ExtensionAccountItemProps) -> Html {
    let account = props.account.clone();
    let onclick = props.onclick.reform(move |_| account.clone());

    html! {
        <li class="account__item">
            <div type="button" class={classes!("relative flex justify-between items-center px-4 py-3 text-gray-600 dark:text-gray-100 hover:bg-gray-200 w-full rounded-md dark:bg-gray-800 cursor-pointer".to_string(), props.highlight.then(|| Some("bg-gray-100")))}
                {onclick}>
                <div class="inline-flex items-center">
                    <Identicon address={props.account.address.clone()} size={24} class="me-2" />
                    <div class="text-start">
                        <p>{props.account.name.clone()}</p>
                        <p class="text-xs">{props.account.to_compact_string()}</p>
                    </div>
                </div>
                // <div class="inline-flex items-center account__item">
                //     <button type="button" class="btn btn__icon" {onclick} >
                //         <svg class="w-3 h-3 text-gray-600 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 4 15">
                //             <path d="M3.5 1.5a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0Zm0 6.041a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0Zm0 5.959a1.5 1.5 0 1 1-3 0 1.5 1.5 0 0 1 3 0Z"/>
                //         </svg>
                //         <span class="sr-only">{"Select"}</span>
                //     </button>
                // </div>
            </div>
        </li>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct ExtensionAccountDropdownProps {
    pub selected: Option<ExtensionAccount>,
    #[prop_or_default]
    pub options: Vec<ExtensionAccount>,
    pub onchange: Callback<ExtensionAccount>,
}

#[function_component(ExtensionAccountDropdown)]
pub fn extension_account_dropdown(props: &ExtensionAccountDropdownProps) -> Html {
    let is_dropdown_hidden = use_state(|| true);

    let btn_dropdown_onclick = {
        let is_dropdown_hidden = is_dropdown_hidden.clone();
        Callback::from(move |_| {
            is_dropdown_hidden.set(!(*is_dropdown_hidden));
        })
    };

    let dropdown_onmouseleave = {
        let is_dropdown_hidden = is_dropdown_hidden.clone();
        Callback::from(move |_| {
            is_dropdown_hidden.set(true);
        })
    };

    let onchange = {
        let is_dropdown_hidden = is_dropdown_hidden.clone();
        props.onchange.reform(move |account: ExtensionAccount| {
            is_dropdown_hidden.set(true);
            account.clone()
        })
    };

    html! {
        <div class="account__dropdown">
            <div class="inline-flex items-center">
                {
                    if props.selected.is_some() {
                        html! {
                            <>
                                <Identicon address={props.selected.as_ref().unwrap().address.clone()} size={24} class="me-2" />
                                <div class="text-start">
                                    <p class="text-gray-900">{props.selected.as_ref().unwrap().name.clone()}</p>
                                    <p class="text-xs">{props.selected.as_ref().unwrap().to_compact_string()}</p>
                                </div>
                            </>
                        }
                    } else {
                        html! {
                            <div class="text-start">{"-- Select the signer account -- "}</div>
                        }
                    }
                }

            </div>
            <div class="inline-flex items-center account__item">
                <button type="button" class="btn btn__icon btn__white" onclick={btn_dropdown_onclick} >
                    {
                        if *is_dropdown_hidden {
                            html! {
                                <>
                                    <svg class="w-3 h-3 text-gray-600 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                                        <path fill-rule="evenodd" d="M18.425 10.271C19.499 8.967 18.57 7 16.88 7H7.12c-1.69 0-2.618 1.967-1.544 3.271l4.881 5.927a2 2 0 0 0 3.088 0l4.88-5.927Z" clip-rule="evenodd"/>
                                    </svg>
                                    <span class="sr-only">{"Open dropdown"}</span>
                                </>
                            }
                        } else {
                            html! {
                                <>
                                    <svg class="w-3 h-3 text-gray-600 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                                        <path fill-rule="evenodd" d="M5.575 13.729C4.501 15.033 5.43 17 7.12 17h9.762c1.69 0 2.618-1.967 1.544-3.271l-4.881-5.927a2 2 0 0 0-3.088 0l-4.88 5.927Z" clip-rule="evenodd"/>
                                    </svg>
                                    <span class="sr-only">{"Close dropdown"}</span>
                                </>
                            }
                        }
                    }
                </button>
            </div>
            <div class={classes!("menu_dropdown", (*is_dropdown_hidden).then(|| Some("hidden")))} role="menu"
                onmouseleave={dropdown_onmouseleave}>
                <ul class="text-sm text-gray-700 dark:text-gray-200">
                    { for props.options.iter().cloned().map(|account| {

                        if props.selected.is_some() {
                            html! {
                                <ExtensionAccountItem account={account.clone()}
                                    highlight={account == props.selected.as_ref().unwrap().clone()} onclick={&onchange} />
                            }
                        } else {
                            html! {
                                <ExtensionAccountItem account={account.clone()} onclick={&onchange} />
                            }
                        }

                    }) }
                </ul>
            </div>
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct FilterItemProps {
    pub filter: Filter,
    pub selected: bool,
    pub onclick: Callback<Filter>,
}

#[function_component(FilterItem)]
pub fn filter_item(props: &FilterItemProps) -> Html {
    let state = use_context::<StateContext>().unwrap();

    let filter = props.filter.clone();

    let mut class = Classes::from("inline-block px-4 py-2 rounded-full min-w-24");

    if props.selected {
        class.push("text-white bg-gray-500 active");
    } else {
        class.push(
            "text-gray-600 dark:text-gray-400 hover:text-gray-900 hover:bg-gray-100 dark:hover:bg-gray-800 dark:hover:text-white",
        );
    }

    let onclick = props.onclick.reform(move |_| match filter {
        Filter::All => filter.clone(),
        Filter::Following(_) => {
            let accounts = state
                .accounts
                .iter()
                .map(|a| AccountId32::from_str(&a.address).unwrap())
                .collect::<Vec<AccountId32>>();
            Filter::Following(accounts)
        }
        Filter::Claimable(_) => {
            if let Some(block_number) = state.network.finalized_block_number {
                if let Some(child_bounties_raw) = &state.child_bounties_raw {
                    let accounts = state
                        .accounts
                        .iter()
                        .map(|a| AccountId32::from_str(&a.address).unwrap())
                        .collect::<Vec<AccountId32>>();
                    let ids = child_bounties_raw
                        .into_iter()
                        .filter(|(_, cb)| {
                            cb.is_claimable(block_number) && accounts.contains(&cb.beneficiary)
                        })
                        .map(|(id, _)| id.clone())
                        .collect::<Vec<Id>>();
                    return Filter::Claimable(ids);
                }
            }
            Filter::Claimable(Vec::new())
        }
    });

    html! {
        <li class="inline-flex ms-2" >
            <button type="button" {class} {onclick}>
                { props.filter.clone() }
            </button>
        </li>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct ChildBountyItemProps {
    pub id: Id,
    #[prop_or_default]
    pub is_action_hidden: bool
}

#[function_component(ChildBountyItem)]
pub fn child_bounty_item(props: &ChildBountyItemProps) -> Html {
    let state = use_context::<StateContext>().unwrap();

    if let Some(child_bounties) = &state.child_bounties_raw {
        if let Some(child_bounty) = child_bounties.get(&props.id) {
            if let Some(block_number) = state.network.finalized_block_number {
                
                let is_already_following = state
                    .accounts
                    .iter()
                    .any(|account| *account.address == child_bounty.beneficiary.to_string());
        
                let onclick = {
                    let state = state.clone();
                    let account = child_bounty.beneficiary.to_string();
                    Callback::from(move |_| {
                        state.dispatch(Action::AddAccount(account.clone()));
                    })
                };

                return html! {
                    <li class="flex rounded-lg bg-white dark:bg-gray-700 hover:highlight">
                        <div class={classes!("flex-auto", "py-3", props.is_action_hidden.then(|| Some("px-6")), (!props.is_action_hidden).then(|| Some("ps-6 pe-2")))}>
                            <div class="flex items-center justify-between">
                                <h4 class="flex-auto text-base text-gray-800 dark:text-gray-200 block truncate w-1">
                                    {child_bounty.description.clone()}
                                </h4>
                                <div class="inline-flex items-center ms-2">
                                    <div class="text-lg text-gray-800 dark:text-gray-200">
                                        {child_bounty.value_human(state.network.runtime)}
                                    </div>
                                    <div class="ml-1 text-lg text-gray-600 dark:text-gray-400">{state.network.runtime.unit()}</div>
                                </div>
                            </div>
                            <div class="inline-flex items-center gap-2">
                                <p class="text-xs">{format!("# {} / {}", child_bounty.parent_id, child_bounty.id)}</p>
                                <SubsquareIconLink id={child_bounty.id.to_string()} runtime={state.network.runtime} />
                                <PolkassemblyIconLink id={child_bounty.id.to_string()} runtime={state.network.runtime} />
                            </div>
                            <hr class="my-2" />
                            <div class="flex items-center justify-between">

                                <AccountChip account={child_bounty.beneficiary.clone()} />

                                { if child_bounty.is_claimable(block_number) {
                                    html! {
                                        <span class={classes!("chip", state.network.runtime.class())}>
                                            {"Claimable"}
                                        </span>
                                    }
                                } else {
                                    html! {
                                        <span class="text-xs">
                                            {format!("Claim in {}", child_bounty.unlock_duration(block_number))}
                                        </span>
                                    }
                                }}

                            </div>
                        </div>
                        <div class={classes!(props.is_action_hidden.then(|| Some("hidden")))}>
                            <div class="w-14 rounded-e-lg h-full flex items-center justify-center">
                                {
                                    if !is_already_following {
                                        html! {
                                            <button type="button" class={classes!("btn", "btn__icon", "btn__transparent")} aria-label="Follow Account"
                                                {onclick} >
                                                <svg class="w-4 h-4 text-inherent dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                                                    <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 12h14m-7 7V5"/>
                                                </svg>
                                                <span class="sr-only">{"Follow"}</span>
                                            </button>
                                        }
                                    } else {
                                        html! {
                                            <svg class="w-4 h-4 text-gray-900 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                                                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="3" d="M5 11.917 9.724 16.5 19 7.5"/>
                                            </svg>
                                        }
                                    }
                                }
                            </div>
                        </div>
                    </li>
                };
            }
        }
    }
    html! {}
}

#[derive(PartialEq, Properties, Clone)]
pub struct ChildBountyItemSmallProps {
    pub id: Id,
}

#[function_component(ChildBountyItemSmall)]
pub fn child_bounty_item_small(props: &ChildBountyItemSmallProps) -> Html {
    let state = use_context::<StateContext>().unwrap();

    if let Some(child_bounties) = &state.child_bounties_raw {
        if let Some(child_bounty) = child_bounties.get(&props.id) {
            return html! {
                <li class="flex rounded-md bg-gray-50 dark:bg-gray-700">
                    <div class="flex-auto p-6 ">
                        <div class="flex items-center justify-between">
                            <h4 class="flex-auto text-sm text-gray-800 dark:text-gray-200 block truncate w-1">
                                {child_bounty.description.clone()}
                            </h4>
                            <div class="inline-flex items-center ms-2">
                                <div class="text-sm text-gray-800 dark:text-gray-200">
                                    {child_bounty.value_human(state.network.runtime)}
                                </div>
                                <div class="ml-1 text-sm text-gray-600 dark:text-gray-400">{state.network.runtime.unit()}</div>
                            </div>
                        </div>
                        <p class="text-xs">{format!("# {} / {}", child_bounty.parent_id, child_bounty.id)}</p>
                        // <hr class="my-2" />
                        // <div class="flex items-center justify-between">

                        //     <AccountChip account={child_bounty.beneficiary.clone()} />

                        // </div>
                    </div>
                </li>
            };
        }
    }
    html! {}
}
