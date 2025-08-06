use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("A9taNFDCefEMJ3UC4HStRVVBRrrbErGqSWwXvWD2n1H1");

#[program]
pub mod airdrop {
    use super::*;

    pub fn airdrop(ctx:Context<Airdrop>,amount:u64) -> Result<()> {
        let from_pubkey=ctx.accounts.sender.to_account_info();
        let to_pubkey=ctx.accounts.recipient.to_account_info();
        let program_id=ctx.accounts.system_program.to_account_info();

        let cpi_context=CpiContext::new(
            program_id,
            Transfer{
                from:from_pubkey,
                to:to_pubkey,
            },
        );
        transfer(cpi_context,amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Airdrop<'info> {
    #[account(mut)]
    sender:Signer<'info>,

    #[account(mut)]
    recipient:SystemAccount<'info>,
    system_program:Program<'info,System>
}
