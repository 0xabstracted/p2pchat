use anchor_lang::prelude::*;

#[repr(C)]
#[account]
pub struct Message{
    pub receiver_address: Pubkey,
    pub sender_address: Pubkey,
    pub message: String,
    pub read_receipt: bool,
    pub bump: u8,
}

const MAX_PUBKEY_LENGTH: usize = 32;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_MESSAGE_LENGTH: usize = 4 * 512;
const MAX_BOOLSIZE: usize = 1;
const MAX_SIZE_IU8: usize = 1;

impl Message{
    pub const LEN: usize = 2 * MAX_PUBKEY_LENGTH
                        + STRING_LENGTH_PREFIX + MAX_MESSAGE_LENGTH
                        + MAX_BOOLSIZE
                        + MAX_SIZE_IU8;
}
