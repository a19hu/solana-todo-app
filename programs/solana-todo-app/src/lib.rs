use anchor_lang::prelude::*;

declare_id!("ErJnoBRAQ9YZYbQ47kmF8nEcr1UL4JFhD9jzSKYErDRx");

#[program]
pub mod solana_todo_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
