use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct ReadMessage<'info>{
    #[account(mut)]
    pub message: Account<'info, Message>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    #[account(constraint = receiver.key() == message.receiver_address)]
    pub receiver: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<ReadMessage>) -> Result<()> {
    let message = &mut ctx.accounts.message;
    message.read_receipt =  true;
    
    Ok(())
}
