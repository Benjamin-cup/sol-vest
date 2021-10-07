pub mod resources;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint, 
    entrypoint::ProgramResult, 
    msg, 
    program::{invoke_signed},
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::create_account,
    sysvar::Sysvar,
};

use crate::{
    resources::bonfida::token_vesting::{
        instruction::{Schedule, VestingInstruction},
        state::{VestingSchedule, VestingScheduleHeader}
    },
};

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = VestingInstruction::unpack(instruction_data)?;

    match instruction {
        VestingInstruction::Init {
            seeds,
            number_of_schedules,
        } => {
            msg!("Instruction: Init");
            process_init(program_id, accounts, seeds, number_of_schedules)
        }
        VestingInstruction::Unlock { seeds } => {
            msg!("Instruction: Unlock");
            process_unlock(program_id, accounts, seeds)
        }
        VestingInstruction::ChangeDestination { seeds } => {
            msg!("Instruction: Change Destination");
            process_change_destination(program_id, accounts, seeds)
        }
        VestingInstruction::Create {
            seeds,
            mint_address,
            destination_token_address,
            schedules,
        } => {
            msg!("Instruction: Create Schedule");
            process_create(
                program_id,
                accounts,
                seeds,
                &mint_address,
                &destination_token_address,
                schedules,
            )
        }
    }
}

fn process_init(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    seeds: [u8; 32],
    number_of_schedules: u32,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let system_program_account = next_account_info(accounts_iter)?;
    let funding_account = next_account_info(accounts_iter)?;
    let vesting_account = next_account_info(accounts_iter)?;
    let rent_account = next_account_info(accounts_iter)?;

    let rent = &Rent::from_account_info(rent_account)?;

    let vesting_account_key = Pubkey::create_program_address(&[&seeds], &program_id).unwrap();

    if vesting_account_key != *vesting_account.key {
        msg!("Provided pda account is invalid");
        return Err(ProgramError::InvalidArgument);
    }

    let state_size = (number_of_schedules as usize) * VestingSchedule::LEN + VestingScheduleHeader::LEN;

    let create_vesting_account_instruction = create_account(
        &funding_account.key,
        &vesting_account_key,
        rent.minimum_balance(state_size),
        state_size as u64,
        &program_id,
    );

    invoke_signed(
        &create_vesting_account_instruction,
        &[
            system_program_account.clone(),
            funding_account.clone(),
            vesting_account.clone(),
        ],
        &[&[&seeds]],
    )?;

    Ok(())
}

fn process_unlock(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _seeds: [u8; 32],
) -> ProgramResult {
    msg!("instruction not supported");
    Ok(())
}

fn process_change_destination(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _seeds: [u8; 32],
) -> ProgramResult {
    msg!("instruction not supported");
    Ok(())
}

fn process_create(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _seeds: [u8; 32],
    _mint_address: &Pubkey,
    _destination_token_address: &Pubkey,
    _schedules: Vec<Schedule>,
) -> ProgramResult {
    msg!("instruction not supported");
    Ok(())
}
