use anchor_lang::prelude::*;

use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct CreateProfile<'info>{
    #[account(init,
            payer = user,
            space = 8 + UserProfile::LEN,
            seeds = [b"user_profile", user.key().as_ref()],
            bump
    )]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct CreateProfileParams {
    pub user_name: String,
    pub user_location: String,
}

pub fn handler(ctx: Context<CreateProfile>, params: CreateProfileParams) -> Result<()> {
    let user_profile = &mut ctx.accounts.user_profile;
    let user = &ctx.accounts.user;
    if params.user_name.chars().count()  > 128 {
        return Err(ErrorCode::NameTooLong.into())
    }
    if params.user_location.chars().count()  > 64 {
        return Err(ErrorCode::LocationTooLong.into())
    }
    user_profile.user_address = user.key();
    user_profile.user_name = params.user_name;  
    user_profile.user_location = params.user_location;  
    user_profile.user_profile_bump =  *ctx.bumps.get("user_profile").unwrap();
    user_profile.likes = 0;
    
    Ok(())
}
