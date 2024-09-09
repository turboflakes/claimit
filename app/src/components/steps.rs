use crate::components::{
    buttons::{BackIconButton, NextIconButton},
    child_bounties::ChildBountiesCard,
    chips::AccountChip,
    inputs::AccountInput,
};
use crate::state::{Action, StateContext};
use gloo::timers::callback::Timeout;
use std::str::FromStr;
use subxt::config::substrate::AccountId32;
use yew::{
    classes, function_component, html, use_context, use_effect_with, use_state, Callback, Html,
};

#[function_component(OnboardingSteps)]
pub fn onboarding_steps() -> Html {
    let step = use_state(|| 0);
    let timeout = use_state(|| None);
    let state = use_context::<StateContext>().unwrap();

    use_effect_with(step.clone(), {
        let state = state.clone();
        move |step| {
            if **step == 2 {
                let handle = Timeout::new(5_000, move || state.dispatch(Action::FinishOnboarding));
                timeout.set(Some(handle));
            }
        }
    });

    let onclick_next = {
        let step = step.clone();
        if *step == 2 {
            let state = state.clone();
            Callback::from(move |_| state.dispatch(Action::FinishOnboarding))
        } else {
            Callback::from(move |_| step.set(*step + 1))
        }
    };

    let onclick_back = {
        let step = step.clone();
        if *step == 0 {
            Callback::from(move |_| step.set(2))
        } else {
            Callback::from(move |_| step.set(*step - 1))
        }
    };

    let onadd = {
        let state = state.clone();
        Callback::from(move |account| {
            state.dispatch(Action::AddAccount(account));
        })
    };

    html! {
        <div class="w-full max-w-[375px] sm:max-w-[828px]">
            <div class="flex mb-4 sm:mb-8">
                <ul>
                    <li class={classes!{"step__index", (*step != 0).then(|| Some("hidden"))}}>
                        <p class="font-black text-9xl">{"1"}</p>
                    </li>
                    <li class={classes!{"step__index", (*step != 1).then(|| Some("hidden"))}}>
                        <p class="font-black text-9xl">{"2"}</p>
                    </li>
                    <li class={classes!{"step__index", (*step != 2).then(|| Some("hidden"))}}>
                        <p class="font-black text-9xl">{"3"}</p>
                    </li>
                </ul>
                <div class="flex flex-col justify-between w-full ms-4 sm:ms-8 h-60">
                    {
                        match *step {
                            0 => html! {
                                <div>
                                    <h1 class="step__title">
                                        {"Follow up on your Child Bounties."}
                                    </h1>
                                    <p class="step__subtitle">
                                        {"Choose from the open Child Bounties below and follow the ones that interest you the most."}
                                    </p>
                                </div>
                            },
                            1 => html! {
                                <div>
                                    <h1 class="step__title">
                                        {"Is your Child Bounty not live yet?"}
                                    </h1>
                                    <p class="step__subtitle">
                                        {"Expecting an award soon? Enter the beneficiary address below."}
                                    </p>
                                </div>
                            },
                            _ => html! {
                                <div>
                                    <h1 class="step__title">
                                        {"You're all set!"}
                                    </h1>
                                    <p class="step__subtitle">
                                        {format!("Following {} account{}. Enjoy and go claim your Child Bounties!", state.accounts.len(), (state.accounts.len() > 1).then_some("s").unwrap_or(""))}
                                    </p>
                                </div>
                            }
                        }
                    }
                    <div>
                        <div class="relative w-full max-w-[254px] sm:max-w-[684px] overflow-auto">
                            <div class="flex flex-nowrap gap-2 items-center py-4">
                                { for state.accounts.iter().cloned().map(|account| html! {
                                    <AccountChip class="bg-white rounded-full px-2" account={AccountId32::from_str(&account.address).unwrap()} removable={true} />
                                })}
                            </div>
                        </div>
                        <div class="inline-flex items-center justify-end w-full">
                            <BackIconButton onclick={onclick_back} disabled={*step == 0} />
                            <ul class="inline-flex mx-4 gap-4">
                                <li class={classes!{"step__dot", "bg-gray-900"}} />
                                <li class={classes!{"step__dot", (*step >= 1).then(|| Some("bg-gray-900")), (*step == 0).then(|| Some("bg-gray-500"))}} />
                                <li class={classes!{"step__dot", (*step >= 2).then(|| Some("bg-gray-900")), (*step <= 1).then(|| Some("bg-gray-500"))}} />
                            </ul>
                            <NextIconButton onclick={onclick_next} disabled={(*step == 1 && state.accounts.len() == 0) || state.network.is_fetching() || state.network.is_initializing()} />
                        </div>
                    </div>
                </div>
            </div>
            {
                match *step {
                    0 => html! {
                        <ChildBountiesCard />
                    },
                    1 => html! {
                        <div class="h-96 p-4 md:p-6 bg-gray-50 text-medium text-gray-500 dark:text-gray-400 dark:bg-gray-800 rounded-lg w-full">
                            <h3 class="md:text-lg font-bold text-gray-900 dark:text-gray-100">{"Add Account"}</h3>
                            <AccountInput onenter={&onadd} placeholder={"Enter the child bounty beneficiary account you wish to keep track of"} />
                        </div>
                    },
                    _ => html! {
                        <div class="h-96 w-full flex items-center justify-center">
                            <img class="w-[256px]" src="/images/claimit_icon_gray.svg" alt="claim.it" />
                        </div>
                    }
                }
            }

        </div>
    }
}
