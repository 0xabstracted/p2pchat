use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
pub struct LikeProfile<'info>{
    #[account(mut)]
    pub user_profile: Account<'info, UserProfile>,
    /// CHECK: This is not dangerous because we don't read or write from this account
    pub visitor: UncheckedAccount<'info>,
}

pub fn handler(ctx: Context<LikeProfile>) -> Result<()> {
    let user_profile = &mut ctx.accounts.user_profile;
    user_profile.likes += 1;
    
    Ok(())
}
