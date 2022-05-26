use anchor_lang::prelude::*;

#[repr(C)]
#[account]
pub struct UserProfile{
    pub user_address: Pubkey,
    pub user_name: String,
    pub user_location: String,
    pub likes: u16,
    pub bump: u8,
}


const MAX_PUBKEY_LENGTH: usize = 32;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_NAME_LENGTH: usize = 4 * 128;
const MAX_LOCATION_LENGTH: usize = 4 * 32;
const MAX_SIZE_IU16: usize = 2;
const MAX_SIZE_IU8: usize = 1;

impl UserProfile{
    pub const LEN: usize = MAX_PUBKEY_LENGTH
                        + STRING_LENGTH_PREFIX + MAX_NAME_LENGTH
                        + STRING_LENGTH_PREFIX + MAX_LOCATION_LENGTH
                        + MAX_SIZE_IU16
                        + MAX_SIZE_IU8;
}
