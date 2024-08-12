use crate::components::{
    buttons::{AddAccountButton, BalanceButtonGroup},
    items::{AccountItem, AccountItemSmall},
};
use crate::state::{Action, StateContext};
use claimeer_common::{
    runtimes::{support::SupportedRelayRuntime, utils::amount_human},
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
            <div class="inline-flex gap-8 w-full items-end">

                {
                    match state.layout.balance_mode.clone() {
                        BalanceMode::TotalBalance => html! { <TotalBalanceTitle runtime={props.runtime.clone()} /> },
                        BalanceMode::TotalAwarded => html! { <TotalAwardedTitle runtime={props.runtime.clone()} /> }
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
