use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LayoutState {
    pub is_add_account_modal_visible: bool,
    pub balance_mode: BalanceMode,
}

impl LayoutState {
    pub fn new() -> Self {
        Self {
            is_add_account_modal_visible: false,
            balance_mode: BalanceMode::TotalBalance,
        }
    }

    pub fn is_total_balance_mode(&self) -> bool {
        self.balance_mode == BalanceMode::TotalBalance
    }

    pub fn is_total_awarded_mode(&self) -> bool {
        self.balance_mode == BalanceMode::TotalAwarded
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BalanceMode {
    TotalBalance,
    TotalAwarded,
}
