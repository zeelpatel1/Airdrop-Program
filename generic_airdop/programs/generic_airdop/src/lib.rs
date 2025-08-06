use anchor_lang::prelude::*;
use solana_program::program::invoke;
use solana_program::{
    program::invoke,
    instruction::{Instruction, AccountMeta},
};

declare_id!("8zxG7No8omDJ1CebgWqWoKfW8EmeUZuucGLEwgBkKHY5");

#[program]
pub mod generic_airdop {
    use super::*;

    pub fn airdrop(ctx:Context<Airdrop>,amount:u64) -> Result<()> {
        let from_pubkey=ctx.accounts.sender.to_account_info();
        let to_pubkey=ctx.accounts.recipient.to_account_info();
        let program_id=ctx.accounts.system_program.to_account_info();


        // SOL transfer instruction inside the system_program
        // 0 -> create_account
        // 1-> assign
        // 2 -> transfer
        // 3 -> allocate
        // ...

        let instruction_discriminator:u32=2;

        // Borsh stores data in form of bytes and in reverse order.
        // i.e we use to_le_bytes()

        let mut instruction_data=Vec::with_capacity(4+8);
        instruction_data.extend_from_slice(&instruction_discriminator.to_le_bytes());
        instruction_data.extend_from_slice(&amount.to_le_bytes());

        let instruction=Instruction {
            program_id:program_id.key(),
            accounts:vec![
                AccountMeta::new(from_pubkey.key(),true),
                AccountMeta::new(to_pubkey.key(),false),
            ],
            data:instruction_data,
        };

        invoke(&instruction,&[from_pubkey,to_pubkey,program_id])?;
        Ok(())
    }

}

#[derive(Accounts)]
pub struct Airdrop<'info>{
    #[account(mut)]
    sender:Signer<'info>,
    #[account(mut)]
    recipient:SystemAccount<'info>,
    system_program:Program<'info,System>,
}