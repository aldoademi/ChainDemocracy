use chrono::{DateTime, Utc, NaiveDateTime};
use solana_program::{
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    account_info::{next_account_info, AccountInfo},
    system_instruction,
    program_error::ProgramError,
    sysvar::{rent::Rent, Sysvar},
    program::invoke_signed,
    borsh::try_from_slice_unchecked,
};

use crate::state::vote_account_state::VoteAccountState;
use borsh::{BorshDeserialize, BorshSerialize};
use crate::utilities::vote_account_utilities;
use crate::pda_management::generate_candidate_list_account;


pub fn add_vote_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    start_date: NaiveDateTime,
    end_date: NaiveDateTime
) -> ProgramResult {

    //Check dates
    // let _ = vote_account_utilities::check_dates(start_date, end_date);
    //----------------------------------------------------------------
    let formatted_start_date = start_date.format("%Y-%m-%d %H:%M:%S").to_string();
    let formatted_end_date = end_date.format("%Y-%m-%d %H:%M:%S").to_string();

     // Get Iterator
     let account_info_iter = &mut accounts.iter();

     // Get accounts
     let initializer = next_account_info(account_info_iter)?;
     let pda_account = next_account_info(account_info_iter)?;
     let system_program = next_account_info(account_info_iter)?;

     let (pda, bump_seed) = Pubkey::find_program_address(
        &[program_id.as_ref(), name.as_bytes().as_ref()],
         program_id
        );

    let account_len: usize = 1 +
     (4 * name.len()) + 
     (4 * formatted_start_date.len()) +
     (4 * formatted_end_date.len()) +
     6 * (32 + 32 * 10);

      // Calculate rent required
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    //Create account
    invoke_signed(
        &system_instruction::create_account(
            initializer.key, 
            pda_account.key, 
            rent_lamports,
            account_len.try_into().unwrap(), 
            program_id
        ), 
        &[initializer.clone(),pda_account.clone(),system_program.clone()], 
        &[&[program_id.as_ref(),name.as_bytes().as_ref(), &[bump_seed]]]
    )?;

    msg!("PDA Created: {}",pda);


    let is_election_created = initialize_vote_account(pda_account, name, formatted_start_date, formatted_end_date);
       
   
    if is_election_created.is_ok(){
        return Ok(())
    } else {
        return Err(ProgramError::AccountBorrowFailed)
    }

    
}

pub fn initialize_vote_account(
    pda_account: &AccountInfo,
    name: String,
    start_date: String,
    end_date: String
) -> ProgramResult {

    msg!("Unpacking vote account");
    let mut account_data = try_from_slice_unchecked::<VoteAccountState>(&pda_account.data.borrow()).unwrap();

    account_data.name = name;
    account_data.start_date = start_date;
    account_data.end_date = end_date;
    account_data.is_initialized = true;
    account_data.is_active = false;

    msg!("Serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("Account serialized");

    Ok(())

}


pub fn try_update(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String
) -> ProgramResult {
    
    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;

    if pda_account.owner != program_id{
        return Err(ProgramError::IllegalOwner);
    }

    // if !initializer.is_signer {
    //     msg!("Missing required signature");
    //     return Err(ProgramError::MissingRequiredSignature)
    // }

    msg!("Unpacking vote account...");
    let mut account_data: VoteAccountState = try_from_slice_unchecked::<VoteAccountState>(&pda_account.data.borrow()).unwrap();

    let(pda, bump_seed) = Pubkey::find_program_address(
        &[program_id.as_ref(),name.as_bytes().as_ref()],
         program_id
        );
    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA");
        return Err(ProgramError::InvalidSeeds)
    }

    //Controllo che l'account sia inizializzato

    msg!("Vote before Update: ");
    msg!("Name: {}", account_data.name);
    msg!("Start date: {}", account_data.start_date);
    msg!("End date: {}", account_data.end_date);
    msg!("Is active: {}", account_data.is_active);

    account_data.is_active = true;

    msg!("Vote after Update: ");
    msg!("Name: {}", account_data.name);
    msg!("Start date: {}", account_data.start_date);
    msg!("End date: {}", account_data.end_date);
    msg!("Is active: {}", account_data.is_active);

    msg!("Serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("Vote account serialized");

    Ok(())

}

// pub fn add_candidate(
//     program_id: &Pubkey,
//     pda_account: &AccountInfo,
//     election_name: String,
//     candidate_first_name: String,
//     candidate_last_name: String
// ) -> ProgramResult {

//     if pda_account.owner != program_id{
//         return Err(ProgramError::IllegalOwner);
//     }

//     let(pda, bump_seed) = Pubkey::find_program_address(
//         &[program_id.as_ref(),election_name.as_bytes().as_ref()],
//          program_id
//         );
//     if pda != *pda_account.key {
//         msg!("Invalid seeds for PDA");
//         return Err(ProgramError::InvalidSeeds)
//     }

//     let mut account_data: VoteAccountState = try_from_slice_unchecked::<VoteAccountState>(&pda_account.data.borrow()).unwrap();

//     if !account_data.is_initialized {
//         msg!("Account not initialized")
//         return Err(ProgramError::InvalidAccountData)
//     }

//     account_data.votes.

//     Ok(())
// }
