use crate::components::items::AccountItem;
use crate::state::{Action, StateContext};
use yew::{function_component, html, use_context, Callback, Html};

#[function_component(AccountsCard)]
pub fn accounts_card() -> Html {
    let state = use_context::<StateContext>().unwrap();

    let onunfollow = {
        let state = state.clone();
        Callback::from(move |e| {
            state.dispatch(Action::Remove(e));
        })
    };

    if state.accounts.len() > 0 {
        html! {
            <div class="mb-4">
                <ul class="flex flex-wrap items-center mx-2 text-xs font-medium text-gray-500 dark:text-gray-400">
                    { for state.accounts.iter().cloned().map(|account|
                        html! {
                            <AccountItem {account}  onunfollow={&onunfollow} />
                    }) }
                </ul>
            </div>
        }
    } else {
        html! {}
    }
}
