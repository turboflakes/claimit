use crate::components::buttons::ActionButton;
use crate::router::Routes;
use yew::{function_component, html, Html};
use yew_router::prelude::use_navigator;

#[function_component(PageNotFound)]
pub fn page_not_found() -> Html {
    let navigator = use_navigator().unwrap();
    let onclick = {
        let navigator = navigator.clone();
        move |_| {
            navigator.push(&Routes::Index);
        }
    };

    html! {
        <div class="page__not_found">
            {"UPPS! PAGE NOT FOUND"}
            <ActionButton label={"go back"} disable={false} {onclick} >
            { "" }
            </ActionButton>
        </div>
    }
}
