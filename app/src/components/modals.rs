use crate::components::{
    buttons::{Button, SignButton},
    inputs::AccountInput,
    items::{ChildBountyItemSmall, ExtensionAccountDropdown},
};
use crate::state::{Action, StateContext};
use claimit_common::types::{
    claims::ClaimStatus,
    extensions::{collect_signature, get_accounts, ExtensionAccount, ExtensionStatus},
};
use log::error;
use yew::{
    classes, function_component, html, platform::spawn_local, use_context, use_effect_with,
    use_state, Callback, Html,
};

#[function_component(ClaimModal)]
pub fn claim_modal() -> Html {
    let is_visible = use_state(|| false);
    let err = use_state(|| "".to_string());
    let extensions_supported = use_state(|| Vec::<Extension>::new());
    let extensions_available = use_state(|| Vec::<Extension>::new());
    let extension_accounts = use_state(|| Vec::<ExtensionAccount>::new());
    let state = use_context::<StateContext>().unwrap();
    let extension = state.extension.clone();

    use_effect_with(state.claim.clone(), {
        let is_visible = is_visible.clone();
        let err = err.clone();
        let state = state.clone();
        let extension = state.extension.clone();

        move |claim| {
            if let Some(claim) = claim.clone() {
                match claim.status {
                    ClaimStatus::Initializing => {
                        is_visible.set(true);
                        if extension.signer.is_some() {
                            state.dispatch(Action::ConnectExtension);
                        }
                    }
                    ClaimStatus::Signing(payload) => {
                        if extension.is_ready() {
                            let signer = extension.signer.as_ref().unwrap().clone();
                            err.set("".to_string());
                            spawn_local(async move {
                                match collect_signature(
                                    payload.clone(),
                                    signer.source.to_string(),
                                    signer.address.clone(),
                                )
                                .await
                                {
                                    Ok(signature) => {
                                        state.dispatch(Action::SubmitWithSignature(signature));
                                    }
                                    Err(e) => {
                                        error!("error: {:?}", e);
                                        err.set(e.to_string());
                                    }
                                }
                            });
                        }
                    }
                    ClaimStatus::Completed => {
                        // TODO: wait 1 or 2 seconds close and dispatch action
                        // TODO: add a green tick on all successfull child bounties in the modal before closing it
                        is_visible.set(false);
                        state.dispatch(Action::ResetClaim);
                    }
                    _ => {}
                }
            }
        }
    });

    use_effect_with(state.extension.clone(), {
        let err = err.clone();
        let extension_accounts = extension_accounts.clone();
        let state = state.clone();

        move |extension| match extension.status {
            ExtensionStatus::Connecting => {
                spawn_local(async move {
                    match get_accounts(source: String).await {
                        Ok(accounts) => {
                            if accounts.len() > 0 {
                                extension_accounts.set(accounts);
                                state.dispatch(Action::ChangeExtensionStatus(
                                    ExtensionStatus::Connected,
                                ));
                            } else {
                                let message = "Please make sure polkadot-js extension is installed and at least one account is enabled to work with this site goclaimit.app";
                                err.set(message.to_string());
                            }
                        }
                        Err(e) => {
                            error!("error: {:?}", e);
                            err.set(e.to_string());
                        }
                    }
                });
            }
            ExtensionStatus::Connected => {
                if let Some(signer) = extension.signer.as_ref() {
                    if extension_accounts.contains(&signer) {
                        state.dispatch(Action::ChangeExtensionStatus(ExtensionStatus::Ready));
                    }
                }
            }
            _ => {}
        }
    });

    let oncancel = {
        let state = state.clone();
        let is_visible = is_visible.clone();
        Callback::from(move |_| {
            is_visible.set(false);
            state.dispatch(Action::ResetClaim);
        })
    };

    let onchange_extension_account = {
        let state = state.clone();
        Callback::from(move |account: ExtensionAccount| {
            state.dispatch(Action::ChangeSigner(account.clone()));
        })
    };

    let onclick_polkadotjs = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(Action::ConnectExtension);
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
                <div class="relative bg-gray-200 rounded-lg shadow dark:bg-gray-700 z-60">
                    <div class="flex items-center justify-between px-4 pt-4 md:px-5 md:pt-5 bg-gray-200 rounded-t-lg">
                        <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
                            {
                                if extension.is_connected_or_ready() {
                                    html! { "Claim child bounties" }
                                } else {
                                    html! { "Connect wallet"}
                                }
                            }
                        </h3>
                        <button type="button" class="btn btn__icon btn__white" onclick={&oncancel} >
                            <svg class="w-4 h-4 text-gray-600 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18 17.94 6M18 18 6.06 6"/>
                            </svg>
                            <span class="sr-only">{"Close"}</span>
                        </button>
                    </div>
                    <div class="p-4 md:p-5 space-y-4">

                        {
                            if extension.is_connected_or_ready() {
                                html! {
                                    <div>
                                        <h4 class="ms-2 mb-2 text-sm text-gray-600 dark:text-gray-100">{"Claim from account"}</h4>
                                        <ExtensionAccountDropdown selected={extension.signer.clone()}
                                            options={(*extension_accounts).clone()} onchange={&onchange_extension_account} />
                                    </div>
                                }
                            } else {

                                html! {
                                    <div>
                                        <h4 class="ms-2 mb-2 text-sm text-gray-600 dark:text-gray-100">{"Supported wallets"}</h4>
                                        <div class="">
                                            <Button label={"Polkadot JS"} class={classes!("btn__logo", "px-4")} onclick={&onclick_polkadotjs} >
                                                <img class="h-6" src="/images/polkadot_js_logo.svg" alt="polkadot js extension" />
                                            </Button>
                                        </div>
                                    </div>
                                }

                            }
                        }

                        {
                            if extension.is_connected_or_ready() && state.claim.is_some() {

                                    html! {
                                        <div>
                                            <h4 class="ms-2 mb-2 text-sm text-gray-600 dark:text-gray-100">{"Claimable child bounties"}</h4>
                                            <ul class="flex-column space-y space-y-4 text-sm font-medium text-gray-600 dark:text-gray-400 overflow-y-scroll h-96">
                                                { for state.claim.clone().unwrap().child_bounty_ids.into_iter().map(|(_, cb_id)|
                                                    html! {
                                                        <ChildBountyItemSmall id={cb_id} />
                                                    })
                                                }
                                            </ul>
                                        </div>
                                    }

                            } else { html! {} }
                        }

                    </div>

                    <div class="flex items-center justify-between p-4 md:p-5 rounded-b-lg">
                        <button type="button" class="btn btn__default" onclick={&oncancel}>{"Cancel"}</button>

                        {
                            if extension.is_connected_or_ready() {
                                html! { <SignButton /> }
                            } else {
                                html! {}
                            }
                        }

                    </div>
                    <div class="ps-6 mt-1 text-sm text-red">{err.to_string()}</div>
                </div>
            </div>
        </div>
    }
}

#[function_component(AddAccountModal)]
pub fn add_account_modal() -> Html {
    let is_visible = use_state(|| false);
    let state = use_context::<StateContext>().unwrap();

    use_effect_with(state.layout.clone(), {
        let is_visible = is_visible.clone();

        move |layout| {
            is_visible.set(layout.is_add_account_modal_visible);
        }
    });

    let onadd = {
        let state = state.clone();
        Callback::from(move |account| {
            state.dispatch(Action::AddAccount(account));
            state.dispatch(Action::ToggleLayoutAddAccountModal);
        })
    };

    let oncancel = {
        let state = state.clone();
        Callback::from(move |_| {
            state.dispatch(Action::ToggleLayoutAddAccountModal);
        })
    };

    let visibility = if *is_visible {
        Some("flex")
    } else {
        Some("hidden")
    };

    html! {
        <div class={classes!("modal__add_account", visibility)}>
            <div class="relative p-4 w-full max-w-2xl max-h-full">
                <div class="relative bg-gray-200 rounded-lg shadow dark:bg-gray-700 z-60">
                    <div class="flex items-center justify-between px-4 pt-4 md:px-5 md:pt-5 rounded-t-lg">
                        <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
                            {"Add Account"}
                        </h3>
                        <button type="button" class="btn btn__icon btn__white" onclick={&oncancel} >
                            <svg class="w-4 h-4 text-gray-600 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                                <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18 17.94 6M18 18 6.06 6"/>
                            </svg>
                            <span class="sr-only">{"Close"}</span>
                        </button>
                    </div>
                    <div class="p-4 md:p-5 space-y-4">

                        {
                            if *is_visible {
                                html! { <AccountInput placeholder="Enter a child bounty beneficiary address" onenter={&onadd} /> }
                            } else {
                                html! {}
                            }
                        }

                    </div>

                    <div class="flex items-center justify-between p-4 md:p-5 rounded-b-lg">

                    </div>
                </div>
            </div>
        </div>
    }
}
