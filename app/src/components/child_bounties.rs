use crate::components::{
    buttons::ClaimButton,
    inputs::FilterInput,
    items::{ChildBountyItem, FilterItem},
    spinners::Spinner,
};
use crate::state::{Action, StateContext};
use claimeer_common::runtimes::utils::amount_human;
use claimeer_common::types::child_bounties::Filter;
use strum::IntoEnumIterator;
use yew::{function_component, html, use_context, use_state, Callback, Html};

#[function_component(ChildBountiesCard)]
pub fn child_bounties_card() -> Html {
    let state = use_context::<StateContext>().unwrap();

    if state.network.is_fetching() && state.child_bounties_raw.is_none() {
        html! {
            <div class="flex justify-center items-center h-96 p-4 md:p-6 bg-gray-50 max-w-[375px] sm:max-w-[828px] rounded-lg w-full">
                <Spinner is_visible={state.network.is_fetching()} />
            </div>
        }
    } else if state.child_bounties_raw.is_some() {
        html! {
            <div class="p-4 md:p-6 bg-gray-50 max-w-[375px] sm:max-w-[828px] text-medium text-gray-500 dark:text-gray-400 dark:bg-gray-800 rounded-lg w-full">

                <ChildBountiesTitle />

                <ChildBountiesBody />

            </div>
        }
    } else {
        html! {}
    }
}

#[function_component(ChildBountiesFilters)]
pub fn child_bounties_filters() -> Html {
    let state = use_context::<StateContext>().unwrap();

    let onclick = {
        let state = state.clone();
        Callback::from(move |e| {
            state.dispatch(Action::SetFilter(e));
        })
    };

    if let Some(child_bounties_raw) = &state.child_bounties_raw {
        let child_bountes_total = child_bounties_raw
            .into_iter()
            .filter(|(_, cb)| state.filter.check(cb))
            .count();

        html! {
            <div class="flex md:mb-4 justify-between items-center ">
                <div class="inline-flex mb-2">
                    <h3 class="md:text-lg text-gray-900 dark:text-gray-100 me-1">{child_bountes_total}</h3>
                    <h3 class="md:text-lg font-bold text-gray-900 dark:text-gray-100">{"Child Bounties"}</h3>
                </div>
                <ul class="tab tab__filters">
                    { for Filter::iter().map(|filter| {
                        html! {
                            <FilterItem filter={filter.clone()}
                                selected={state.filter.to_string() == filter.to_string()}
                                onclick={&onclick}
                            />
                        }
                    }) }
                </ul>
            </div>
        }
    } else {
        html! {}
    }
}

#[function_component(ChildBountiesTitle)]
pub fn child_bounties_title() -> Html {
    let state = use_context::<StateContext>().unwrap();

    if let Some(child_bounties_raw) = &state.child_bounties_raw {
        let child_bountes_total = child_bounties_raw
            .into_iter()
            .filter(|(_, cb)| state.filter.check(cb))
            .count();

        html! {
            <div class="flex justify-between items-center mb-4">
                <div class="inline-flex">
                    <h3 class="md:text-lg text-gray-900 dark:text-gray-100 me-1">{child_bountes_total}</h3>
                    <h3 class="md:text-lg font-bold text-gray-900 dark:text-gray-100">{"Child Bounties"}</h3>
                </div>

                {
                    if state.filter.is_following() {
                        html!{ <ClaimButton /> }
                    } else {
                        html! {}
                    }
                }

            </div>
        }
    } else {
        html! {}
    }
}

#[function_component(ChildBountiesStats)]
pub fn child_bounties_stats() -> Html {
    let state = use_context::<StateContext>().unwrap();
    let runtime = state.network.runtime.clone();

    if let Some(child_bounties_raw) = &state.child_bounties_raw {
        if let Some(block_number) = state.network.finalized_block_number {
            let amount_pending = child_bounties_raw
                .into_iter()
                .filter(|(_, cb)| state.filter.check(cb) && !cb.is_claimable(block_number))
                .map(|(_, cb)| cb.value)
                .sum::<u128>();

            let amount_claimable = child_bounties_raw
                .into_iter()
                .filter(|(_, cb)| state.filter.check(cb) && cb.is_claimable(block_number))
                .map(|(_, cb)| cb.value)
                .sum::<u128>();

            return html! {
                <div class="mb-4">
                    <hr/>
                    <div class="flex mx-2 my-2 justify-center items-center">
                        {
                            if !state.filter.is_claimable() {
                                html! {
                                    <div class="text-center me-8">
                                        <div class="text-xs font-light dark:text-gray-100">{"Total Pending"}</div>
                                        <div class="inline-flex">
                                            <span class="text-xs font-bold dark:text-gray-100 me-1">{amount_human(amount_pending, runtime.decimals().into())}</span>
                                            <span class="text-xs dark:text-gray-100">{runtime.unit()}</span>
                                        </div>
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }
                        <div class="text-center">
                            <div class="text-xs font-light dark:text-gray-100">{"Total Claimable"}</div>
                            <div class="inline-flex">
                                <span class="text-xs font-bold dark:text-gray-100 me-1">{amount_human(amount_claimable, runtime.decimals().into())}</span>
                                <span class="text-xs dark:text-gray-100">{runtime.unit()}</span>
                            </div>
                        </div>

                    </div>
                    <hr/>
                </div>
            };
        }
    }
    html! {}
}

#[function_component(ChildBountiesBody)]
pub fn child_bounties_body() -> Html {
    let state = use_context::<StateContext>().unwrap();
    let input_value = use_state(|| "".to_string());

    let oninput = {
        let state = state.clone();
        let input_value = input_value.clone();
        Callback::from(move |value| {
            input_value.set(value);
        })
    };

    if let Some(child_bounties_raw) = &state.child_bounties_raw {
        html! {
            <>
                <FilterInput oninput={&oninput} value={(*input_value).clone()} placeholder="Filter by Child Bounty description" />

                <ul class="flex-col w-full space-y space-y-4 text-sm font-medium text-gray-500 dark:text-gray-400">
                    {
                        for child_bounties_raw.into_iter()
                            .filter(|(_, cb)| state.filter.check(cb) && cb.description.to_lowercase().contains(&(*input_value).to_lowercase()))
                            .map(|(_, cb)|
                        html! {
                            <ChildBountyItem id={cb.id} />
                        })
                    }
                </ul>
            </>
        }
    } else {
        html! {}
    }
}
