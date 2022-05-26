pub use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;
pub mod error;

use instructions::*;


declare_id!("8rgXH6wodLMnkcDMkdpVLMTbnDQUjDaHgAAhw7BuysQE");

#[program]
pub mod p_to_p_chat {
    use super::*;

    pub fn create_profile(ctx: Context<CreateProfile>, params: CreateProfileParams ) -> Result<()> {
        instructions::create_profile::handler(ctx, params)
    }
    pub fn like_profile(ctx: Context<LikeProfile>) -> Result<()> {
        instructions::like_profile::handler(ctx)
    }
    pub fn read_message(ctx: Context<ReadMessage>) ->Result<()> {
        instructions::read_message::handler(ctx)
    }
    pub fn send_message(ctx: Context<SendMessage>, params: SendMessageParams) -> Result<()> {
        instructions::send_message::handler(ctx, params)
    }
    pub fn update_profile(ctx: Context<UpdateProfile>, params: UpdateProfileParams) -> Result<()> {
        instructions::update_profile::handler(ctx, params)
    }
}
