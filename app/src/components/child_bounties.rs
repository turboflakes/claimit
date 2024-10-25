use crate::components::{
    buttons::{BountyAllToggle, BountyIdToggle, ClaimButton},
    inputs::FilterInput,
    items::{ChildBountyItem, FilterItem},
    spinners::Spinner,
};
use crate::state::{Action, StateContext};
use claimit_common::runtimes::utils::amount_human;
use claimit_common::types::child_bounties::Filter;
use std::collections::BTreeSet;
use strum::IntoEnumIterator;
use yew::{function_component, html, use_context, use_state, Callback, Html, UseStateHandle, Properties};

#[function_component(ChildBountiesCard)]
pub fn child_bounties_card() -> Html {
    let state = use_context::<StateContext>().unwrap();
    let bounties_filter: UseStateHandle<BTreeSet<u32>> = use_state(|| BTreeSet::new());

    let ontoggle = {
        let bounties_filter = bounties_filter.clone();

        Callback::from(move |value: u32| {
            let mut tmp: BTreeSet<u32> = (*bounties_filter).clone();
            if tmp.contains(&value) {
                tmp.remove(&value);
            } else {
                tmp.insert(value);
            }
            bounties_filter.set(tmp);
        })
    };

    let ontoggle_all = {
        let bounties_filter = bounties_filter.clone();

        Callback::from(move |_| {
            let tmp: BTreeSet<u32> = (*bounties_filter).clone();
            if tmp.len() > 0 {
                bounties_filter.set(BTreeSet::new());
            }
        })
    };

    if (state.network.is_initializing() || state.network.is_fetching())
        && state.child_bounties_raw.is_none()
    {
        html! {
            <div class="flex flex-col justify-center items-center h-96 p-4 md:p-6 bg-gray-50 max-w-[375px] sm:max-w-[828px] rounded-lg w-full">
                <Spinner is_visible={state.network.is_initializing() || state.network.is_fetching()} />
                {
                    if state.network.is_initializing() {
                        html! {<p class="mt-4 text-sm text-center">{state.network.is_initializing_description()}</p>}
                    } else {
                        html! {<p class="mt-4 text-sm text-center">{state.network.is_fetching_description()}</p>}
                    }
                }

            </div>
        }
    } else if let Some(child_bounties_raw) = &state.child_bounties_raw {

        let mut all_bounties = child_bounties_raw
            .into_iter()
            .map(|(_, cb)| cb.parent_id)
            .collect::<Vec<u32>>();

        all_bounties.sort();
        all_bounties.dedup();

        html! {
            <div class="flex flex-col min-h-96 p-4 md:p-6 bg-gray-50  max-w-[375px] sm:max-w-[828px] rounded-lg w-full">

                {
                    if state.layout.is_onboarding {
                        html! {
                            <div class="flex flex-nowrap gap-2 items-center mb-4 ms-1">
                                <BountyAllToggle onclick={&ontoggle_all} selected={(*bounties_filter).len() == 0} />
                                { for all_bounties.iter().cloned().map(|id| html! {
                                    <BountyIdToggle id={id} onclick={&ontoggle} selected={(*bounties_filter).contains(&id)} />
                                })}
                            </div>
                        }
                    } else { html! {} }
                }

                <ChildBountiesTitle bounties_filter={(*bounties_filter).clone()} />

                <ChildBountiesBody bounties_filter={(*bounties_filter).clone()} />

            </div>
        }
    } else {
        html! {}
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct ChildBountiesTitleProps {
    pub bounties_filter: BTreeSet<u32>,
}

#[function_component(ChildBountiesTitle)]
pub fn child_bounties_title(props: &ChildBountiesTitleProps) -> Html {
    let state = use_context::<StateContext>().unwrap();

    if let Some(child_bounties_raw) = &state.child_bounties_raw {
        let child_bounties_total = child_bounties_raw
            .into_iter()
            .filter(|(_, cb)| state.filter.check(cb) && (props.bounties_filter.len() == 0 || props.bounties_filter.contains(&cb.parent_id)))
            .count();

        if child_bounties_total > 0 {
            return html! {
                <div class="flex flex-none justify-between items-center mb-4 ms-1">
                    <div class="inline-flex items-center">
                        <h3 class="md:text-lg text-gray-900 dark:text-gray-100 me-1">{child_bounties_total}</h3>
                        <h3 class="md:text-lg font-bold text-gray-900 dark:text-gray-100 me-4">{"Child Bounties"}</h3>
                        <Spinner is_visible={state.network.is_fetching()} />
                    </div>

                    {
                        if state.filter.is_following() {
                            html!{ <ClaimButton /> }
                        } else {
                            html! {}
                        }
                    }

                </div>
            };
        }
    }
    html! {}
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

#[derive(PartialEq, Properties, Clone)]
pub struct ChildBountiesBodyProps {
    pub bounties_filter: BTreeSet<u32>,
}

#[function_component(ChildBountiesBody)]
pub fn child_bounties_body(props: &ChildBountiesBodyProps) -> Html {
    let state = use_context::<StateContext>().unwrap();
    let input_value = use_state(|| "".to_string());

    let oninput = {
        let input_value = input_value.clone();
        Callback::from(move |value| {
            input_value.set(value);
        })
    };

    let onclear = {
        let input_value = input_value.clone();
        Callback::from(move |_| {
            input_value.set("".to_string());
        })
    };

    if let Some(child_bounties_raw) = &state.child_bounties_raw {
        let child_bountes_total = child_bounties_raw
            .into_iter()
            .filter(|(_, cb)| state.filter.check(cb))
            .count();

        html! {
            <>
                {
                    if state.layout.is_onboarding {
                        html! {
                            <FilterInput value={(*input_value).clone()} placeholder="Filter by Child Bounty description"
                                oninput={&oninput} onclear={&onclear}/>
                        }
                    } else { html! {} }
                }

                {
                    if child_bountes_total > 0 {
                        html! {
                            <ul class="flex-col w-full space-y space-y-4 text-sm font-medium text-gray-500 dark:text-gray-400">
                                {
                                    for child_bounties_raw.into_iter()
                                        .filter(|(_, cb)| state.filter.check(cb) && (cb.description.to_lowercase().contains(&(*input_value).to_lowercase()) && (props.bounties_filter.len() == 0 || props.bounties_filter.contains(&cb.parent_id))))
                                        .map(|(_, cb)|
                                    html! {
                                        <ChildBountyItem id={cb.id} is_action_hidden={!state.layout.is_onboarding} />
                                    })
                                }
                            </ul>
                        }
                    } else if state.network.is_fetching() {
                        html! {
                            <div class="flex flex-col flex-1 justify-center items-center">
                                <Spinner is_visible={true} />
                                <p class="mt-4 text-xs text-center">{"Searching for any open child bounties awarded to the accounts you follow. Hang tight..."}</p>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="flex flex-col flex-1 justify-center items-center">
                                <p class="mt-4 text-xs text-center">{"There are no open child bounties awarded to the accounts you follow."}</p>
                            </div>
                        }
                    }
                }
            </>
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