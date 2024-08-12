use crate::components::{
    buttons::{AddAccountButton, BalanceButtonGroup},
    items::{AccountItem, AccountItemSmall},
};
use crate::state::{Action, StateContext};
use claimeer_common::{
    runtimes::{support::SupportedRelayRuntime, utils::amount_human},
    types::accounts::Account,
    types::{child_bounties::Filter, layout::BalanceMode},
};
use std::str::FromStr;
use subxt::utils::AccountId32;
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

    html! {
        <div class="relative w-full max-w-[375px] md:max-w-[828px] overflow-auto">
            <ul class="flex flex-nowrap gap-4 items-center py-4 text-xs font-medium text-gray-500 dark:text-gray-400">
                { for state.accounts.iter().rev().cloned().map(|account|
                    html! {
                        <AccountItem {account} runtime={props.runtime.clone()} onunfollow={&onunfollow} />
                }) }

                <li class="account__item">
                    <AddAccountButton />
                </li>

            </ul>
        </div>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct TotalBalancesCardProps {
    pub runtime: SupportedRelayRuntime,
}

#[function_component(TotalBalancesCard)]
pub fn total_balances_card(props: &TotalBalancesCardProps) -> Html {
    let state = use_context::<StateContext>().unwrap();

    html! {
        <div class="w-full max-w-[375px] md:max-w-[828px]">
            <div class="inline-flex gap-8 w-full items-end justify-between">

                {
                    match state.layout.balance_mode.clone() {
                        BalanceMode::TotalBalance => html! { <TotalBalanceTitle runtime={props.runtime.clone()} /> },
                        BalanceMode::TotalAwarded => html! { <TotalAwardedTitle runtime={props.runtime.clone()} /> },
                        BalanceMode::TotalPending => html! { <TotalPendingTitle runtime={props.runtime.clone()} /> },
                        BalanceMode::TotalClaimable => html! { <TotalClaimableTitle runtime={props.runtime.clone()} /> }
                    }

                }

                <BalanceButtonGroup />

            </div>
        </div>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct TotalBalanceTitleProps {
    pub runtime: SupportedRelayRuntime,
}

#[function_component(TotalBalanceTitle)]
pub fn total_balance_title(props: &TotalBalanceTitleProps) -> Html {
    let state = use_context::<StateContext>().unwrap();

    let total_balance = state
        .accounts
        .iter()
        .map(|account| account.balance.total())
        .sum::<u128>();

    if total_balance > 0 {
        html! {
            <div>
                <p>{"Total balance"}</p>
                <div class="inline-flex">
                    <h3 class="font-medium text-2xl text-gray-900">
                        {amount_human(total_balance, props.runtime.decimals().into())}
                        <span class="text-gray-600 ms-2">{props.runtime.unit()}</span>
                    </h3>
                </div>
            </div>
        }
    } else {
        html! {}
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct TotalAwardedTitleProps {
    pub runtime: SupportedRelayRuntime,
}

#[function_component(TotalAwardedTitle)]
pub fn total_awarded_title(props: &TotalAwardedTitleProps) -> Html {
    let state = use_context::<StateContext>().unwrap();

    let following = state
        .accounts
        .iter()
        .map(|a| AccountId32::from_str(&a.address).unwrap())
        .collect::<Vec<AccountId32>>();

    let filter = Filter::Following(following);

    if let Some(child_bounties_raw) = &state.child_bounties_raw {
        let total_awarded = child_bounties_raw
            .into_iter()
            .filter(|(_, cb)| filter.check(cb))
            .map(|(_, cb)| cb.value)
            .sum::<u128>();

        return html! {
            <div>
                <p>{"Total awarded"}</p>
                <h3 class="font-medium text-2xl text-gray-900">
                    {amount_human(total_awarded, props.runtime.decimals().into())}
                    <span class="text-gray-600 ms-2">{props.runtime.unit()}</span>
                </h3>
            </div>
        };
    }
    html! {}
}

#[derive(PartialEq, Properties, Clone)]
pub struct TotalPendingTitleProps {
    pub runtime: SupportedRelayRuntime,
}

#[function_component(TotalPendingTitle)]
pub fn total_pending_title(props: &TotalPendingTitleProps) -> Html {
    let state = use_context::<StateContext>().unwrap();

    let following = state
        .accounts
        .iter()
        .map(|a| AccountId32::from_str(&a.address).unwrap())
        .collect::<Vec<AccountId32>>();

    let filter = Filter::Following(following);

    if let Some(child_bounties_raw) = &state.child_bounties_raw {
        if let Some(block_number) = state.network.finalized_block_number {
            let total_pending = child_bounties_raw
                .into_iter()
                .filter(|(_, cb)| filter.check(cb) && !cb.is_claimable(block_number))
                .map(|(_, cb)| cb.value)
                .sum::<u128>();

            return html! {
                <div>
                    <p>{"Total pending"}</p>
                    <h3 class="font-medium text-2xl text-gray-900">
                        {amount_human(total_pending, props.runtime.decimals().into())}
                        <span class="text-gray-600 ms-2">{props.runtime.unit()}</span>
                    </h3>
                </div>
            };
        }
    }
    html! {}
}

#[derive(PartialEq, Properties, Clone)]
pub struct TotalClaimableTitleProps {
    pub runtime: SupportedRelayRuntime,
}

#[function_component(TotalClaimableTitle)]
pub fn total_claimable_title(props: &TotalClaimableTitleProps) -> Html {
    let state = use_context::<StateContext>().unwrap();

    let following = state
        .accounts
        .iter()
        .map(|a| AccountId32::from_str(&a.address).unwrap())
        .collect::<Vec<AccountId32>>();

    let filter = Filter::Following(following);

    if let Some(child_bounties_raw) = &state.child_bounties_raw {
        if let Some(block_number) = state.network.finalized_block_number {
            let total_claimable = child_bounties_raw
                .into_iter()
                .filter(|(_, cb)| filter.check(cb) && cb.is_claimable(block_number))
                .map(|(_, cb)| cb.value)
                .sum::<u128>();

            return html! {
                <div>
                    <p>{"Total claimable"}</p>
                    <h3 class="font-medium text-2xl text-gray-900">
                        {amount_human(total_claimable, props.runtime.decimals().into())}
                        <span class="text-gray-600 ms-2">{props.runtime.unit()}</span>
                    </h3>
                </div>
            };
        }
    }
    html! {}
}

#[derive(PartialEq, Properties, Clone)]
pub struct AccountBalanceProps {
    pub account: Account,
    pub runtime: SupportedRelayRuntime,
}

#[function_component(AccountBalance)]
pub fn account_balance(props: &AccountBalanceProps) -> Html {
    let state = use_context::<StateContext>().unwrap();

    html! {
        <div>
            {
                match state.layout.balance_mode.clone() {
                    BalanceMode::TotalBalance => html! {
                        <div class="inline-flex items-center">
                            <svg class="w-5 h-5 text-gray-600 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                                <path fill-rule="evenodd" d="M12 14a3 3 0 0 1 3-3h4a2 2 0 0 1 2 2v2a2 2 0 0 1-2 2h-4a3 3 0 0 1-3-3Zm3-1a1 1 0 1 0 0 2h4v-2h-4Z" clip-rule="evenodd"/>
                                <path fill-rule="evenodd" d="M12.293 3.293a1 1 0 0 1 1.414 0L16.414 6h-2.828l-1.293-1.293a1 1 0 0 1 0-1.414ZM12.414 6 9.707 3.293a1 1 0 0 0-1.414 0L5.586 6h6.828ZM4.586 7l-.056.055A2 2 0 0 0 3 9v10a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2h-4a5 5 0 0 1 0-10h4a2 2 0 0 0-1.53-1.945L17.414 7H4.586Z" clip-rule="evenodd"/>
                            </svg>
                            <p class="text-xl text-gray-800 ms-3">{props.account.balance.total_human(props.runtime.clone())}</p>
                        </div>
                    },
                    BalanceMode::TotalAwarded =>{

                        if let Some(child_bounties_raw) = &state.child_bounties_raw {
                            let total_awarded = child_bounties_raw
                                .into_iter()
                                .filter(|(_, cb)| props.account.child_bounty_ids.contains(&cb.id))
                                .map(|(_, cb)| cb.value)
                                .sum::<u128>();

                            return html! {
                                <div class="inline-flex items-center">
                                    <svg class="w-5 h-5 text-gray-600 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M11 9a1 1 0 1 1 2 0 1 1 0 0 1-2 0Z"/>
                                        <path fill-rule="evenodd" d="M9.896 3.051a2.681 2.681 0 0 1 4.208 0c.147.186.38.282.615.255a2.681 2.681 0 0 1 2.976 2.975.681.681 0 0 0 .254.615 2.681 2.681 0 0 1 0 4.208.682.682 0 0 0-.254.615 2.681 2.681 0 0 1-2.976 2.976.681.681 0 0 0-.615.254 2.682 2.682 0 0 1-4.208 0 .681.681 0 0 0-.614-.255 2.681 2.681 0 0 1-2.976-2.975.681.681 0 0 0-.255-.615 2.681 2.681 0 0 1 0-4.208.681.681 0 0 0 .255-.615 2.681 2.681 0 0 1 2.976-2.975.681.681 0 0 0 .614-.255ZM12 6a3 3 0 1 0 0 6 3 3 0 0 0 0-6Z" clip-rule="evenodd"/>
                                        <path d="M5.395 15.055 4.07 19a1 1 0 0 0 1.264 1.267l1.95-.65 1.144 1.707A1 1 0 0 0 10.2 21.1l1.12-3.18a4.641 4.641 0 0 1-2.515-1.208 4.667 4.667 0 0 1-3.411-1.656Zm7.269 2.867 1.12 3.177a1 1 0 0 0 1.773.224l1.144-1.707 1.95.65A1 1 0 0 0 19.915 19l-1.32-3.93a4.667 4.667 0 0 1-3.4 1.642 4.643 4.643 0 0 1-2.53 1.21Z"/>
                                    </svg>
                                    <p class="text-xl text-gray-800 ms-3">{amount_human(total_awarded, props.runtime.decimals().into())}</p>
                                </div>
                            }
                        }
                        html! {}
                    },
                    BalanceMode::TotalPending =>{

                        if let Some(child_bounties_raw) = &state.child_bounties_raw {
                            if let Some(block_number) = state.network.finalized_block_number {
                                let total_pending = child_bounties_raw
                                    .into_iter()
                                    .filter(|(_, cb)| props.account.child_bounty_ids.contains(&cb.id) && !cb.is_claimable(block_number))
                                    .map(|(_, cb)| cb.value)
                                    .sum::<u128>();

                                return html! {
                                    <div class="inline-flex items-center">
                                        <svg class="w-5 h-5 text-gray-600 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                                            <path fill-rule="evenodd" d="M10 5a2 2 0 0 0-2 2v3h2.4A7.48 7.48 0 0 0 8 15.5a7.48 7.48 0 0 0 2.4 5.5H5a2 2 0 0 1-2-2v-7a2 2 0 0 1 2-2h1V7a4 4 0 1 1 8 0v1.15a7.446 7.446 0 0 0-1.943.685A.999.999 0 0 1 12 8.5V7a2 2 0 0 0-2-2Z" clip-rule="evenodd"/>
                                            <path fill-rule="evenodd" d="M10 15.5a5.5 5.5 0 1 1 11 0 5.5 5.5 0 0 1-11 0Zm6.5-1.5a1 1 0 1 0-2 0v1.5a1 1 0 0 0 .293.707l1 1a1 1 0 0 0 1.414-1.414l-.707-.707V14Z" clip-rule="evenodd"/>
                                        </svg>
                                        <p class="text-xl text-gray-800 ms-3">{amount_human(total_pending, props.runtime.decimals().into())}</p>
                                    </div>
                                }
                            }
                        }
                        html! {}
                    }
                    BalanceMode::TotalClaimable =>{

                        if let Some(child_bounties_raw) = &state.child_bounties_raw {
                            if let Some(block_number) = state.network.finalized_block_number {
                                let total_claimable = child_bounties_raw
                                    .into_iter()
                                    .filter(|(_, cb)| props.account.child_bounty_ids.contains(&cb.id) && cb.is_claimable(block_number))
                                    .map(|(_, cb)| cb.value)
                                    .sum::<u128>();

                                return html! {
                                    <div class="inline-flex items-center">
                                        <svg class="w-5 h-5 text-gray-600 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="currentColor" viewBox="0 0 24 24">
                                            <path fill-rule="evenodd" d="M14 4.182A4.136 4.136 0 0 1 16.9 3c1.087 0 2.13.425 2.899 1.182A4.01 4.01 0 0 1 21 7.037c0 1.068-.43 2.092-1.194 2.849L18.5 11.214l-5.8-5.71 1.287-1.31.012-.012Zm-2.717 2.763L6.186 12.13l2.175 2.141 5.063-5.218-2.141-2.108Zm-6.25 6.886-1.98 5.849a.992.992 0 0 0 .245 1.026 1.03 1.03 0 0 0 1.043.242L10.282 19l-5.25-5.168Zm6.954 4.01 5.096-5.186-2.218-2.183-5.063 5.218 2.185 2.15Z" clip-rule="evenodd"/>
                                        </svg>
                                        <p class="text-xl text-gray-800 ms-3">{amount_human(total_claimable, props.runtime.decimals().into())}</p>
                                    </div>
                                }
                            }
                        }
                        html! {}
                    }
                }

            }
        </div>
    }
}
