use plot_icon::generate_svg;
use std::str::FromStr;
use subxt::utils::AccountId32;
use yew::{function_component, html, AttrValue, Html, Properties};

#[derive(PartialEq, Properties, Clone)]
pub struct IdenticonProps {
    // address is an ss58-encoded address or publicKey
    pub address: AttrValue,
    // size (optional) is a number, indicating the size in pixel (default 32)
    #[prop_or(32_u16)]
    pub size: u16,
    // class (optional) optional classes
    #[prop_or_default]
    pub class: AttrValue,
}

#[function_component(Identicon)]
pub fn identicon(props: &IdenticonProps) -> Html {
    // Generate Identicon svg
    let account = AccountId32::from_str(&props.address).unwrap();
    let identicon = generate_svg(&account.0);
    let identicon_parsed = Html::from_html_unchecked(AttrValue::from(identicon.to_string()));
    // <div class={props.class.clone()} style={format!("max-width: {}px; max-height: {}px;", props.size, props.size)}>
    html! {
        <div class={props.class.clone()} style={format!("width: {}px; height: {}px;", props.size, props.size)}>
            {identicon_parsed}
        </div>
    }
}
