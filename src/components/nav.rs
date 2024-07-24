use crate::components::{
    buttons::{ClaimButton, NetworkSubscriber},
    spinners::Spinner,
};
use crate::runtimes::support::SupportedRelayRuntime;
use crate::state::StateContext;
use num_format::{Locale, ToFormattedString};
use yew::{function_component, html, use_context, Callback, Html, Properties};

#[derive(PartialEq, Properties, Clone)]
pub struct NavbarProps {
    pub runtime: SupportedRelayRuntime,
}

#[function_component(Navbar)]
pub fn navbar(props: &NavbarProps) -> Html {
    let state = use_context::<StateContext>().unwrap();

    html! {
        <nav class="fixed w-full z-20 top-0 start-0 bg-transparent dark:bg-gray-900">
            <div class="flex flex-wrap items-center justify-between mx-4 p-4">
                <a href="https://claimeer.app/">
                    <img class="h-8" src="/images/claimeer_logo_gray_600.svg" alt="Claimeer logo" />
                </a>

                <div class="inline-flex items-center">

                    <Spinner is_visible={state.network.is_fetching()} />

                    <div class="inline-flex items-center space-x-2">
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
                                }
                            }
                        }
                    </div>

                    <ClaimButton />
                </div>
            </div>
        </nav>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct FooterProps {
    pub runtime: SupportedRelayRuntime,
    pub disabled: bool,
    pub onchange: Callback<SupportedRelayRuntime>,
}

#[function_component(Footer)]
pub fn footer(props: &FooterProps) -> Html {
    let onchange = props.onchange.reform(move |e| e);

    html! {
        <footer class="fixed w-full z-20 bottom-0 start-0 bg-transparent dark:bg-gray-900">
            <div class="flex flex-wrap items-center justify-between mx-4 p-4">
                <div class="flex items-center sm:justify-center sm:mt-0">
                <span class="text-sm text-gray-500 sm:text-center dark:text-gray-400">{"© 2024 Claimeer · Built by "}
                    <a href="https://turboflakes.io/" target="_blank" class="hover:underline hover:underline-offset-2 hover:text-gray-900">{"Turboflakes"}</a>
                </span>
                <a href="https://github.com/turboflakes/claimeer" target="_blank" class="text-gray-500 hover:text-gray-900 dark:hover:text-white ms-5">
                    <svg class="w-4 h-4" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="currentColor" viewBox="0 0 20 20">
                            <path fill-rule="evenodd" d="M10 .333A9.911 9.911 0 0 0 6.866 19.65c.5.092.678-.215.678-.477 0-.237-.01-1.017-.014-1.845-2.757.6-3.338-1.169-3.338-1.169a2.627 2.627 0 0 0-1.1-1.451c-.9-.615.07-.6.07-.6a2.084 2.084 0 0 1 1.518 1.021 2.11 2.11 0 0 0 2.884.823c.044-.503.268-.973.63-1.325-2.2-.25-4.516-1.1-4.516-4.9A3.832 3.832 0 0 1 4.7 7.068a3.56 3.56 0 0 1 .095-2.623s.832-.266 2.726 1.016a9.409 9.409 0 0 1 4.962 0c1.89-1.282 2.717-1.016 2.717-1.016.366.83.402 1.768.1 2.623a3.827 3.827 0 0 1 1.02 2.659c0 3.807-2.319 4.644-4.525 4.889a2.366 2.366 0 0 1 .673 1.834c0 1.326-.012 2.394-.012 2.72 0 .263.18.572.681.475A9.911 9.911 0 0 0 10 .333Z" clip-rule="evenodd"/>
                    </svg>
                    <span class="sr-only">{"GitHub account"}</span>
                </a>
                </div>
                <div class="flex items-center sm:justify-center sm:mt-0">
                    <NetworkSubscriber selected={props.runtime.clone()} disabled={props.disabled.clone()} {onchange} />
                </div>
            </div>
        </footer>
    }
}
