use std::str::FromStr;
use subxt::utils::AccountId32;
use web_sys::{HtmlInputElement, MouseEvent};
use yew::{
    events::KeyboardEvent, function_component, html, use_effect_with, use_node_ref, use_state,
    AttrValue, Callback, Html, InputEvent, Properties, TargetCast,
};

#[derive(PartialEq, Properties, Clone)]
pub struct InputProps {
    pub onenter: Callback<String>,
    #[prop_or_default]
    pub placeholder: AttrValue,
}

#[function_component(AccountInput)]
pub fn account_input(props: &InputProps) -> Html {
    let input_node_ref = use_node_ref();
    let err = use_state(|| "".to_string());

    use_effect_with(input_node_ref.clone(), |input_ref| {
        if let Some(input) = input_ref.cast::<HtmlInputElement>() {
            let _ = input.focus();
        }
    });

    let onkeypress = {
        let onenter = props.onenter.clone();
        let err = err.clone();

        move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input = e.target_unchecked_into::<HtmlInputElement>();
                let value = input.value();
                // Validate if input is a valid SS58 account
                match AccountId32::from_str(&value) {
                    Ok(account) => {
                        input.set_value("");
                        err.set("".to_string());
                        onenter.emit(account.to_string());
                    }
                    Err(_) => {
                        err.set("Invalid SS58 Acccount".to_string());
                    }
                }
            }
        }
    };

    let onmouseover = |e: MouseEvent| {
        e.target_unchecked_into::<HtmlInputElement>()
            .focus()
            .unwrap_or_default();
    };

    let onclick = {
        let input_node_ref = input_node_ref.clone();
        let onenter = props.onenter.clone();
        let err = err.clone();

        move |_| {
            if let Some(input) = input_node_ref.cast::<HtmlInputElement>() {
                let value = input.value();
                match AccountId32::from_str(&value) {
                    Ok(account) => {
                        input.set_value("");
                        err.set("".to_string());
                        onenter.emit(account.to_string());
                    }
                    Err(_) => {
                        err.set("Invalid SS58 Acccount".to_string());
                    }
                }
            }
        }
    };

    html! {
        <div class="my-2 py-4 w-full">
            <div class="relative">
                <div class="absolute inset-y-0 start-1 flex items-center ps-3 pointer-events-none">
                    <svg class="w-4 h-4 text-gray-600 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 20">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z"/>
                    </svg>
                </div>
                <input ref={input_node_ref} type="text" class="account__input" placeholder={props.placeholder.to_string()}
                    {onkeypress} {onmouseover} />
                <div class="absolute inset-y-0 end-0 flex items-center pe-3">
                    <button type="button" class="btn btn__icon white" {onclick} >
                        <svg class="w-4 h-4 text-gray-600 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14m-7 7V5"/>
                        </svg>
                        <span class="sr-only">{"Add Account"}</span>
                    </button>
                </div>
            </div>
            <div class="ps-6 mt-1 text-xs text-red">{err.to_string()}</div>
        </div>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct FilterInputProps {
    pub oninput: Callback<String>,
    pub onclear: Callback<()>,
    pub value: AttrValue,
    #[prop_or_default]
    pub placeholder: AttrValue,
}

#[function_component(FilterInput)]
pub fn filter_input(props: &FilterInputProps) -> Html {
    let input_node_ref = use_node_ref();

    use_effect_with(input_node_ref.clone(), |input_ref| {
        if let Some(input) = input_ref.cast::<HtmlInputElement>() {
            let _ = input.focus();
        }
    });

    let onmouseover = |e: MouseEvent| {
        e.target_unchecked_into::<HtmlInputElement>()
            .focus()
            .unwrap_or_default();
    };

    let oninput = {
        let oninput = props.oninput.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into::<HtmlInputElement>();
            oninput.emit(input.value());
        })
    };

    let onclear = props.onclear.reform(move |_| ());

    html! {
        <div class="mb-4 w-full">
            <div class="relative">
                <div class="absolute inset-y-0 start-1 flex items-center ps-3 pointer-events-none">
                    <svg class="w-4 h-4 text-gray-500 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 18h6v-2H3v2zM3 6v2h18V6H3zm0 7h12v-2H3v2z"/>
                    </svg>
                </div>
                <input ref={input_node_ref} value={props.value.to_string()} type="text" class="filter__input" placeholder={props.placeholder.to_string()}
                    {oninput} {onmouseover} />
                {
                    if props.value.len() > 0 {
                        html!{
                            <div class="absolute inset-y-0 end-0 flex items-center pe-2">
                                <button type="button" class="btn btn__icon_sm white" onclick={onclear} >
                                    <svg class="w-4 h-4 text-gray-800 dark:text-white" aria-hidden="true" xmlns="http://www.w3.org/2000/svg" width="24" height="24" fill="none" viewBox="0 0 24 24">
                                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18 17.94 6M18 18 6.06 6"/>
                                    </svg>
                                </button>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        </div>
    }
}
