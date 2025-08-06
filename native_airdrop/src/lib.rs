use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    system_instruction,
    program::{invoke},
    msg,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id:&Pubkey,
    accounts:&[AccountInfo],
    instruction_data:&[u8]
) -> ProgramResult {

    msg!("Starting native airdrop program...");

    let accounts_iter = &mut accounts.iter();
    let sender = next_account_info(accounts_iter)?;
    let recipient = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let amount = instruction_data
    .get(..8)
    .map(|slice| u64::from_le_bytes(slice.try_into().unwrap()))
    .ok_or(solana_program::program_error::ProgramError::InvalidInstructionData)?;
    
    msg!("Transferring {} lamports from sender to recipient", amount);

    let ix=system_instruction::transfer(sender.key,recipient.key,amount);

    invoke(
        &ix,
        &[
            sender.clone(),
            recipient.clone(),
            system_program.clone(),
        ]
    )?;

    msg!("Transfer successful");
    Ok(())

}