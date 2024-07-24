use crate::components::items::{
    ChildBountyItemSmall, ExtensionAccountDropdown, ExtensionAccountItem,
};
use crate::state::Action;
use crate::state::StateContext;
use crate::types::{
    accounts::{get_accounts, ExtensionAccount},
    claims::ClaimStatus,
};
use log::{error, info};
use yew::{
    classes, function_component, html, platform::spawn_local, use_context, use_effect_with,
    use_state, Callback, Html,
};

#[function_component(ClaimModal)]
pub fn claim_modal() -> Html {
    let is_visible = use_state(|| false);
    let err = use_state(|| "".to_string());
    let extension_accounts = use_state(|| Vec::<ExtensionAccount>::new());
    let state = use_context::<StateContext>().unwrap();
    let signer = state.signer.clone();

    use_effect_with(state.claim.clone(), {
        let is_visible = is_visible.clone();
        let err = err.clone();
        let extension_accounts = extension_accounts.clone();

        move |claim| {
            info!("claim state changed! {:?}", claim);

            if let Some(claim) = claim {
                match claim.status {
                    ClaimStatus::Initializing => {
                        info!("__Initializing");
                        is_visible.set(true);
                        // if let Some(signer) = &claim.signer {
                        //     info!("__Start Signing");
                        // } else {
                        info!("fetching extension accounts");
                        spawn_local(async move {
                            match get_accounts().await {
                                Ok(accounts) => extension_accounts.set(accounts),
                                Err(e) => {
                                    error!("error: {:?}", e);
                                    err.set(e.to_string())
                                }
                            }
                        });
                        // }
                    }
                    _ => todo!(),
                }
            }
        }
    });

    let oncancel = {
        let state = state.clone();
        let is_visible = is_visible.clone();
        Callback::from(move |_| {
            is_visible.set(false);
            state.dispatch(Action::CancelClaim);
        })
    };

    let onclick_extension_account = {
        let state = state.clone();
        Callback::from(move |account: ExtensionAccount| {
            state.dispatch(Action::ChangeSigner(account.clone()));
        })
    };

    let onconfirm = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(Action::SignClaim);
        })
    };

    let visibility = if *is_visible {
        Some("flex")
    } else {
        Some("hidden")
    };

    html! {
        <div class={classes!("modal__claim", visibility)}>
            <div class="relative p-4 w-full max-w-2xl max-h-full">
                <div class="relative bg-white rounded-lg shadow dark:bg-gray-700 z-60">
                    <div class="flex items-center justify-between px-4 pt-4 md:px-5 md:pt-5 rounded-t-lg">
                        <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
                            {"Claim child bounties"}
                        </h3>
                    </div>
                    <div class="p-4 md:p-5 space-y-4">

                        {
                            if signer.is_some() {
                                html! {
                                    <div>
                                        <h4 class="ms-2 mb-2 text-sm text-gray-500 dark:text-gray-100">{"Claimer account"}</h4>
                                        <ExtensionAccountDropdown selected={signer.clone().unwrap()}
                                            options={(*extension_accounts).clone()} onchange={&onclick_extension_account} />
                                    </div>
                                }
                            } else {
                                html! {
                                    <div>
                                        <h4 class="ms-2 mb-2 text-sm text-gray-500 dark:text-gray-100">{"Select claimer account"}</h4>
                                        <ul class="flex-column space-y space-y-4 text-sm font-medium bg-gray-50 text-gray-500 dark:text-gray-400 md:me-4 mb-4 md:mb-0 max-h-60 overflow-y-auto">
                                            { for extension_accounts.iter().cloned().map(|account| {
                                                html! {
                                                    <ExtensionAccountItem account={account.clone()} onclick={&onclick_extension_account} />
                                                }
                                            }) }
                                        </ul>
                                    </div>
                                }
                            }
                        }

                        {
                            // TODO: create a separate component
                            if signer.is_some() && state.claim.is_some() {

                                    html! {
                                        <div>
                                            <h4 class="ms-2 mb-2 text-sm text-gray-500 dark:text-gray-100">{"Claimable child bounties"}</h4>
                                            <ul class="flex-column space-y space-y-4 text-sm font-medium text-gray-500 dark:text-gray-400 overflow-y-scroll h-96">
                                                { for state.claim.clone().unwrap().child_bounty_ids.into_iter().map(|id|
                                                    html! {
                                                        <ChildBountyItemSmall id={id} />
                                                    })
                                                }
                                            </ul>
                                        </div>
                                    }

                            } else { html! {} }
                        }

                    </div>

                    <div class="flex items-center justify-between p-4 md:p-5 bg-gray-50 rounded-b-lg">
                        <button type="button" class="btn btn__default" onclick={oncancel}>{"Cancel"}</button>
                        {
                            if signer.is_some() {
                                html! { <button type="button" class="btn btn__primary" onclick={onconfirm}>{"Sign and Submit"}</button> }
                            } else { html! {} }
                        }
                    </div>
                    <div class="ps-6 mt-1 text-sm text-red">{err.to_string()}</div>
                </div>
            </div>
        </div>
    }
}
