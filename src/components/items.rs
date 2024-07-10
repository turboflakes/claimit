use crate::state::{Account as Item, StateContext};
use yew::{function_component, html, use_context, Callback, Classes, Html, Properties};

#[derive(PartialEq, Properties, Clone)]
pub struct AccountItemProps {
    pub account: Item,
    pub ontoggle: Callback<usize>,
    pub onremove: Callback<usize>,
}

#[function_component(AccountItem)]
pub fn account(props: &AccountItemProps) -> Html {
    let id = props.account.id;
    let mut class = Classes::from("todo");

    if props.account.disabled {
        class.push("disabled");
    }

    let ontoggle = {
        let ontoggle = props.ontoggle.clone();
        move |_| ontoggle.emit(id)
    };

    let onremove = {
        let onremove = props.onremove.clone();
        move |_| onremove.emit(id)
    };

    html! {
        <li {class}>
            <div class="row">
                <span>{props.account.ss58.to_string()}</span>
                <button class="destroy" onclick={onremove} />
                <button class="disable" onclick={ontoggle} />
                <ul class="cbs__list">
                    { for props.account.child_bounty_ids.iter().cloned().map(|id|
                        html! {
                            <ChildBountyItem {id} />
                    }) }
                </ul>
            </div>
        </li>
    }
}

#[derive(PartialEq, Properties, Clone)]
pub struct ChildBountyItemProps {
    pub id: u32,
}

#[function_component(ChildBountyItem)]
pub fn child_bounty(props: &ChildBountyItemProps) -> Html {
    let _state = use_context::<StateContext>().unwrap();

    let class = Classes::from("todo");

    html! {
        <li {class}>
            <div class="row">
                <span>{props.id.to_string()}</span>
            </div>
        </li>
    }
}
