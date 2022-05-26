use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct SendMessage<'info>{
    #[account(init,
            payer = sender,
            space = 8 + Message::LEN,
            seeds = [b"message", sender.key().as_ref()],
            bump
    )]
    pub message: Account<'info, Message>,
    #[account(mut)]
    pub sender: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct SendMessageParams {
    pub receiver_address: Pubkey,
    pub message: String,
}

pub fn handler(ctx: Context<SendMessage>, params: SendMessageParams) -> Result<()> {
    let message = &mut ctx.accounts.message;
    let sender = &ctx.accounts.sender;
    if params.message.chars().count()  > 128 {
        return Err(ErrorCode::MessageTooLong.into())
    }
    message.sender_address = sender.key();
    message.receiver_address = params.receiver_address;  
    message.message = params.message;  
    message.read_receipt =  false;
    message.bump = *ctx.bumps.get("message").unwrap();
    Ok(())
}
