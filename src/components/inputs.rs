use std::str::FromStr;
use web_sys::{HtmlInputElement, MouseEvent};
use yew::{
    events::KeyboardEvent, function_component, html, use_state, AttrValue, Callback, Html,
    Properties, TargetCast,
};

use subxt::utils::AccountId32;

#[derive(PartialEq, Properties, Clone)]
pub struct InputProps {
    pub onenter: Callback<String>,
    pub placeholder: AttrValue,
}

#[function_component(AccountInput)]
pub fn account_input(props: &InputProps) -> Html {
    let error = use_state(|| "".to_string());

    let onkeypress = {
        let onenter = props.onenter.clone();
        let error = error.clone();

        move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: HtmlInputElement = e.target_unchecked_into();
                let value = input.value();
                // Validate if input is a valid SS58 account
                if AccountId32::from_str(&value).is_ok() {
                    input.set_value("");
                    error.set("".to_string());
                    onenter.emit(value);
                } else {
                    error.set("Invalid SS58 Acccount".to_string());
                }
            }
        }
    };

    let onmouseover = |e: MouseEvent| {
        e.target_unchecked_into::<HtmlInputElement>()
            .focus()
            .unwrap_or_default();
    };

    html! {
        <div class="action__input">
            <input class="account" placeholder={props.placeholder.to_string()}
                {onkeypress}
                {onmouseover}
            />
            <span class="error">{error.to_string()}</span>
        </div>
    }
}
