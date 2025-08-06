use anchor_lang::prelude::*;

declare_id!("8zxG7No8omDJ1CebgWqWoKfW8EmeUZuucGLEwgBkKHY5");

#[program]
pub mod generic_airdop {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
