use anchor_lang::prelude::*;
pub mod states;
use crate::states::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("FPAnZULyMwvgVQiNZjesbzoyhz89iynACe9CssuPtLsr");

#[program]
mod hello_anchor {
    use super::*;
    pub fn user_initialize(ctx:Context<UserProfileInitialize>)->Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;
        user_profile.authority = ctx.accounts.authority.key();
        user_profile.todo_count = 0;
        user_profile.last_count = 0;

        Ok(())
    }

    
}

#[derive(Accounts)]
pub struct UserProfileInitialize<'info>{
    #[account(mut)]
    pub authority : Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = UserProfile::LEN,
        seeds = [authority.key().as_ref()],
        bump
    )]
    pub user_profile: Account<'info, UserProfile>,

    pub system_program: Program<'info, System>,
}