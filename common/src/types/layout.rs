use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LayoutState {
    pub is_add_account_modal_visible: bool,
}

impl LayoutState {
    pub fn new() -> Self {
        Self {
            is_add_account_modal_visible: false,
        }
    }
}
