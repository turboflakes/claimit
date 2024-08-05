use crate::components::{
    buttons::{Button, SignButton},
    items::{ChildBountyItemSmall, ExtensionAccountDropdown},
};
use crate::state::{Action, StateContext};
use claimeer_common::runtimes::support::SupportedRelayRuntime;
use claimeer_common::types::{
    child_bounties::ChildBountiesKeys,
    claims::ClaimStatus,
    extensions::{get_accounts, ExtensionAccount, ExtensionStatus},
};
use claimeer_kusama::kusama;
use claimeer_polkadot::polkadot;
use log::{error, info};
use subxt::{OnlineClient, PolkadotConfig};
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
    let extension = state.extension.clone();

    use_effect_with(state.claim.clone(), {
        let is_visible = is_visible.clone();
        let err = err.clone();
        let state = state.clone();
        let extension = state.extension.clone();

        move |claim| {
            info!("claim state changed! {:?}", claim);

            if let Some(claim) = claim {
                match claim.status {
                    ClaimStatus::Initializing => {
                        is_visible.set(true);
                        if extension.signer.is_some() {
                            state.dispatch(Action::ConnectExtension);
                        }
                    }
                    ClaimStatus::Signing => {
                        if extension.is_ready() {
                            err.set("".to_string());
                            let runtime = state.network.runtime.clone();
                            let signer = extension.signer.as_ref().unwrap().clone();
                            let child_bounties_keys = state
                                .claim
                                .as_ref()
                                .unwrap()
                                .child_bounty_ids
                                .iter()
                                .map(|id| {
                                    state
                                        .child_bounties_raw
                                        .as_ref()
                                        .unwrap()
                                        .get(id)
                                        .unwrap()
                                        .key()
                                })
                                .collect::<ChildBountiesKeys>();

                            let claim = claim.clone();
                            spawn_local(async move {
                                let api = OnlineClient::<PolkadotConfig>::from_url(
                                    runtime.default_rpc_url(),
                                )
                                .await
                                .expect("expect valid RPC connection");

                                let res = match runtime {
                                    SupportedRelayRuntime::Polkadot => {
                                        polkadot::create_and_sign_tx(
                                            &api.clone(),
                                            signer.clone(),
                                            child_bounties_keys.clone(),
                                        )
                                        .await
                                    }
                                    SupportedRelayRuntime::Kusama => {
                                        kusama::create_and_sign_tx(
                                            &api.clone(),
                                            signer.clone(),
                                            child_bounties_keys.clone(),
                                        )
                                        .await
                                    }
                                };
                                match res {
                                    Ok((_, extrinsic)) => {
                                        state.dispatch(Action::SubmitClaim(claim));
                                        // TODO; call submit_and_watch
                                    }
                                    Err(e) => {
                                        error!("error: {:?}", e);
                                        err.set(e.to_string());
                                        state.dispatch(Action::ErrorClaim(claim));
                                    }
                                }
                            });
                        }
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

        move |extension| {
            info!("extension state changed! {:?}", extension);

            match extension.status {
                ExtensionStatus::Connecting => {
                    info!("checking pjs extension and fetch enabled accounts");
                    spawn_local(async move {
                        match get_accounts().await {
                            Ok(accounts) => {
                                if accounts.len() > 0 {
                                    extension_accounts.set(accounts);
                                    state.dispatch(Action::ChangeExtensionStatus(
                                        ExtensionStatus::Connected,
                                    ));
                                } else {
                                    let message = "Please make sure polkadot-js extension is installed and at least one account is enabled to work with this site claimeer.app";
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
                <div class="relative bg-white rounded-lg shadow dark:bg-gray-700 z-60">
                    <div class="flex items-center justify-between px-4 pt-4 md:px-5 md:pt-5 rounded-t-lg">
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
                                        <h4 class="ms-2 mb-2 text-sm text-gray-500 dark:text-gray-100">{"Claim from account"}</h4>
                                        <ExtensionAccountDropdown selected={extension.signer.clone()}
                                            options={(*extension_accounts).clone()} onchange={&onchange_extension_account} />
                                    </div>
                                }
                            } else {

                                html! {
                                    <div>
                                        <h4 class="ms-2 mb-2 text-sm text-gray-500 dark:text-gray-100">{"Supported wallets"}</h4>
                                        <div class="">
                                            <Button label={"Polkadot JS"} class="btn__logo" onclick={&onclick_polkadotjs} >
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
