use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct LayoutState {
    pub is_add_account_modal_visible: bool,
    pub is_onboarding: bool,
    pub balance_mode: BalanceMode,
}

impl LayoutState {
    pub fn new(is_onboarding: bool) -> Self {
        Self {
            is_add_account_modal_visible: false,
            is_onboarding,
            balance_mode: BalanceMode::TotalBalance,
        }
    }

    pub fn is_total_balance_mode(&self) -> bool {
        self.balance_mode == BalanceMode::TotalBalance
    }

    pub fn is_total_awarded_mode(&self) -> bool {
        self.balance_mode == BalanceMode::TotalAwarded
    }

    pub fn is_total_pending_mode(&self) -> bool {
        self.balance_mode == BalanceMode::TotalPending
    }

    pub fn is_total_claimable_mode(&self) -> bool {
        self.balance_mode == BalanceMode::TotalClaimable
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum BalanceMode {
    TotalBalance,
    TotalAwarded,
    TotalPending,
    TotalClaimable,
}
