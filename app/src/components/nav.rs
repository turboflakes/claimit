use crate::components::{
    buttons::{NetworkProviderIconButton, NetworkSubscriber},
    spinners::Spinner,
};
use crate::state::StateContext;
use claimit_common::runtimes::support::SupportedRelayRuntime;
use num_format::{Locale, ToFormattedString};
use yew::{function_component, html, use_context, Callback, Html, Properties};

#[derive(PartialEq, Properties, Clone)]
pub struct NavbarProps {
    pub runtime: SupportedRelayRuntime,
    pub ontoggle_provider: Callback<()>,
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    let state = use_context::<StateContext>().unwrap();

    html! {
        <nav class="fixed w-full z-20 top-0 start-0 bg-gradient-to-l from-white from-20%">
            <div class="flex items-center justify-between sm:justify-end px-2 sm:p-4 h-12">
                <a class="block sm:hidden" href="https://goclaimit.app/">
                    <img class="w-6" src="/images/claimit_icon_brand.svg" alt="claim.it" />
                </a>

                <div class="inline-flex items-center">

                    {
                        if state.network.is_ligh_client() && state.network.is_initializing() {
                            html! {
                                <p class="hidden sm:flex text-xs">{"Synchronizing light client..."}</p>
                            }
                        } else {
                            html! {}
                        }

                    }

                    <Spinner is_visible={state.network.is_fetching()} />

                    <div class="ms-4 inline-flex items-center space-x-2 text-gray-900">
                        {
                            if state.network.finalized_block_number.is_some() {
                                html! {
                                    <span class="text-sm">{format!("#{}", state.network.finalized_block_number.unwrap().to_formatted_string(&Locale::en))}</span>
                                }
                            } else { html! {} }
                        }

                        {
                            match props.runtime.clone() {
                                SupportedRelayRuntime::Polkadot => html! {
                                    <>
                                        <img class="h-8" src="/images/polkadot_icon.svg" alt="polkadot logo" />
                                        <span>{"Polkadot"}</span>
                                    </>
                                },
                                SupportedRelayRuntime::Kusama => html! {
                                    <>
                                        <img class="h-8" src="/images/kusama_icon.svg" alt="kusama logo" />
                                        <span>{"Kusama"}</span>
                                    </>
                                },
                                SupportedRelayRuntime::Rococo => html! {
                                    <>
                                        <img class="h-8" src="/images/rococo_icon.svg" alt="rococo logo" />
                                        <span>{"Rococo"}</span>
                                    </>
                                },
                            }
                        }
                    </div>

                    <NetworkProviderIconButton class="ms-4" onclick={props.ontoggle_provider.clone()} />

                </div>
            </div>
        </nav>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct FooterProps {
    pub runtime: SupportedRelayRuntime,
    pub onchange: Callback<SupportedRelayRuntime>,
}

#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    let state = use_context::<StateContext>().unwrap();
    let onchange = props.onchange.reform(move |e| e);

    html! {
        <footer class="w-full my-8 sm:my-2 dark:bg-gray-900">
            <div class="grid grid-cols-1 2xl:grid-cols-3 gap-4 content-center">
                <div class="inline-flex items-center justify-center 2xl:justify-start col-span-2 sm:mt-0 order-last 2xl:order-first">
                    <span class="inline-flex text-xs text-gray-800 sm:text-center dark:text-gray-400">
                        <span class="me-1">{"© 2024 claim.it — Built by"}</span>
                        <a href="https://turboflakes.io/" target="_blank" class="flex items-center hover:underline hover:underline-offset-2 hover:text-gray-900">
                            <span class="me-4">{"Turboflakes"}</span>
                            <svg class="w-4 h-4" viewBox="0 0 60 60" fill="currentColor" xmlns="http://www.w3.org/2000/svg">
                                <circle fill-rule="evenodd" cx="30" cy="30" r="30"></circle>
                                <path d="M20,46 C17.2399393,46 15,43.7600607 15,41 C15,38.2399393 17.2399393,36 20,36 C22.7600607,36 25,38.2399393 25,41 C25,43.7600607 22.7600607,46 20,46 Z M32.0751821,35 C29.3566227,35 18,30 18,30 C18,30 29.3566227,25 32.0751821,25 C34.7937414,25 37,27.2399393 37,30 C37,32.7600607 34.7974808,35 32.0751821,35 Z M39.777954,24 C36.8953212,24 20,19 20,19 C20,19 36.8913561,14 39.777954,14 C42.6645519,14 45,16.2399393 45,19 C45,21.7600607 42.6645519,24 39.777954,24 Z" fill="#EAEDF0"></path>
                            </svg>
                        </a>
                    </span>
                    <a href="https://github.com/turboflakes/claimit" target="_blank" class="text-gray-800 hover:text-gray-900 dark:hover:text-white ms-4">
                        <svg class="w-4 h-4" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M10 .333A9.911 9.911 0 0 0 6.866 19.65c.5.092.678-.215.678-.477 0-.237-.01-1.017-.014-1.845-2.757.6-3.338-1.169-3.338-1.169a2.627 2.627 0 0 0-1.1-1.451c-.9-.615.07-.6.07-.6a2.084 2.084 0 0 1 1.518 1.021 2.11 2.11 0 0 0 2.884.823c.044-.503.268-.973.63-1.325-2.2-.25-4.516-1.1-4.516-4.9A3.832 3.832 0 0 1 4.7 7.068a3.56 3.56 0 0 1 .095-2.623s.832-.266 2.726 1.016a9.409 9.409 0 0 1 4.962 0c1.89-1.282 2.717-1.016 2.717-1.016.366.83.402 1.768.1 2.623a3.827 3.827 0 0 1 1.02 2.659c0 3.807-2.319 4.644-4.525 4.889a2.366 2.366 0 0 1 .673 1.834c0 1.326-.012 2.394-.012 2.72 0 .263.18.572.681.475A9.911 9.911 0 0 0 10 .333Z" clip-rule="evenodd"/>
                        </svg>
                        <span class="sr-only">{"GitHub account"}</span>
                    </a>
                </div>
                <div class="flex items-center justify-center 2xl:justify-end sm:mt-0">
                    <NetworkSubscriber selected={props.runtime.clone()} disabled={state.network.is_initializing()} {onchange} />
                </div>
            </div>
        </footer>
    }
}
