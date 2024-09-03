use plot_icon::generate_svg;
use std::str::FromStr;
use subxt::utils::AccountId32;
use yew::{classes, function_component, html, AttrValue, Callback, Html, Properties};
use yew_hooks::use_clipboard;

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
    let clipboard = use_clipboard();
    // Generate Identicon svg
    let account = AccountId32::from_str(&props.address).unwrap();
    let identicon = generate_svg(&account.0);
    let identicon_parsed = Html::from_html_unchecked(identicon.to_string().into());

    let onclick = {
        let clipboard = clipboard.clone();
        let address = props.address.clone();

        Callback::from(move |_| {
            clipboard.write_text(address.to_string());
        })
    };

    html! {
        <div class={classes!(props.class.clone(), "hover:cursor-copy")}
            style={format!("width: {}px; height: {}px;", props.size, props.size)} {onclick}>
            {identicon_parsed}
        </div>
    }
}
