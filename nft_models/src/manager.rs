use enum_dispatch::enum_dispatch;
use near_contract_standards::non_fungible_token::TokenId;

#[enum_dispatch]
pub trait Manager {
    fn take_parent(&mut self) -> Option<TokenId>;
    fn take_slot(&mut self, token_id: &TokenId) -> Option<TokenId>;
    fn clear_slots(&mut self) -> Option<Vec<TokenId>>;
    fn slots_id(&self) -> Option<Vec<TokenId>>;

    // fn insert_slot(&mut self, token_id: &TokenId) -> bool;
}
