use crate::components::{chips::AccountChip, icons::Identicon};
use crate::state::StateContext;
use crate::types::{
    accounts::{Account, ExtensionAccount},
    child_bounties::{ChildBounty, Filter, Id},
};
use humantime::format_duration;
use std::str::FromStr;
use std::time::Duration;
use subxt::config::substrate::AccountId32;
use yew::{
    classes, function_component, html, use_context, use_state, Callback, Classes, Html, MouseEvent,
    Properties,
};

#[derive(PartialEq, Properties, Clone)]
pub struct AccountItemProps {
    pub account: Account,
    // pub ontoggle: Callback<usize>,
    pub onunfollow: Callback<usize>,
}

#[function_component(AccountItem)]
pub fn account(props: &AccountItemProps) -> Html {
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
        id
    });

    html! {
        <li class="account__item">
            <div class="relative flex justify-between items-center px-4 py-3 rounded-lg text-gray-600 dark:text-gray-100 bg-gray-50 w-full dark:bg-gray-800">
                <div class="inline-flex items-center">
                    <Identicon address={props.account.address.clone()} size={24} class="me-2" />
                    {props.account.to_compact_string()}
                </div>
                <div class="inline-flex items-center account__item">
                    <button type="button" class="btn btn__icon" onclick={btn_dropdown_onclick} >
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
            <div type="button" class={classes!("relative flex justify-between items-center px-4 py-3 text-gray-600 dark:text-gray-100 hover:bg-gray-200 w-full rounded-lg dark:bg-gray-800 cursor-pointer".to_string(), props.highlight.then(|| Some("bg-gray-100")))}
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
    pub selected: ExtensionAccount,
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
                <Identicon address={props.selected.address.clone()} size={24} class="me-2" />
                <div class="text-start">
                    <p>{props.selected.name.clone()}</p>
                    <p class="text-xs">{props.selected.to_compact_string()}</p>
                </div>
            </div>
            <div class="inline-flex items-center account__item">
                <button type="button" class="btn btn__icon" onclick={btn_dropdown_onclick} >
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
                        html! {
                            <ExtensionAccountItem account={account.clone()}
                                highlight={account == props.selected.clone()} onclick={&onchange} />
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
    pub child_bounty: ChildBounty,
}

#[function_component(ChildBountyItem)]
pub fn child_bounty_item(props: &ChildBountyItemProps) -> Html {
    let state = use_context::<StateContext>().unwrap();

    let duration = if let Some(block_number) = state.network.finalized_block_number {
        if !props.child_bounty.is_claimable(block_number) {
            let n = props.child_bounty.unlock_at - block_number;
            let d = Duration::new(n as u64 * 6, 0);
            format_duration(d).to_string()
        } else {
            "".to_string()
        }
    } else {
        "".to_string()
    };

    html! {
        <li class="flex rounded-lg bg-white dark:bg-gray-700">
            <div class="flex-auto p-6 ">
                <div class="flex items-center justify-between">
                    <h4 class="flex-auto text-base text-gray-800 dark:text-gray-200 block truncate w-1">
                        {props.child_bounty.description.clone()}
                    </h4>
                    <div class="inline-flex items-center ms-2">
                        <div class="text-lg text-gray-800 dark:text-gray-200">
                            {props.child_bounty.value_human(state.network.runtime)}
                        </div>
                        <div class="ml-1 text-lg text-gray-600 dark:text-gray-400">{state.network.runtime.unit()}</div>
                    </div>
                </div>
                <p class="text-xs">{format!("# {} / {}", props.child_bounty.parent_id, props.child_bounty.id)}</p>
                <hr class="my-2" />
                <div class="flex items-center justify-between">

                    <AccountChip account={props.child_bounty.beneficiary.clone()} />

                    { if state.network.finalized_block_number.is_some() && props.child_bounty.is_claimable(state.network.finalized_block_number.unwrap()) {
                        html! {
                            <span class="bg-yellow text-gray-900 text-xs font-medium px-2.5 py-1 rounded-full">
                                {"Claimable"}
                            </span>
                        }
                    } else {
                        html! {
                            <span class="text-xs">
                                {format!("Claim in {}", duration)}
                            </span>
                        }
                    }}

                </div>
            </div>
        </li>
    }
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
                <li class="flex rounded-lg bg-gray-50 dark:bg-gray-700">
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
