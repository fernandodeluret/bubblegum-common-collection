use anchor_lang::prelude::*;

#[account]
pub struct CollectionAuthority {
    pub bump: u8,
}
