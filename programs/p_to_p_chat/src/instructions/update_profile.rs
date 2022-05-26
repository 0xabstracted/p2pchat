use anchor_lang::prelude::*;
use crate::state::*;
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct UpdateProfile<'info>{
    #[account(
        mut, 
        seeds = [b"user_profile", user.key().as_ref()], 
        bump = user_profile.bump
    )]
    pub user_profile: Account<'info, UserProfile>, 
    pub user: Signer<'info>,
}


#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct UpdateProfileParams {
    pub user_name: String,
    pub user_location: String,
}

pub fn handler(ctx: Context<UpdateProfile>, params: UpdateProfileParams) -> Result<()>{
    let user_profile = &mut ctx.accounts.user_profile;
    if params.user_name.chars().count() > 128 {
        return Err(ErrorCode::NameTooLong.into())
    }
    if params.user_location.chars().count() > 64 {
        return Err(ErrorCode::LocationTooLong.into())
    }
    user_profile.user_name = params.user_name;
    user_profile.user_location = params.user_location;
    Ok(())
}