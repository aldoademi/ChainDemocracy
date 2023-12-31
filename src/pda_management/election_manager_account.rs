use chrono::NaiveDateTime;
use solana_program::{
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    account_info::{next_account_info, AccountInfo},
    system_instruction,
    program_error::ProgramError,
    sysvar::{rent::Rent, Sysvar},
    program::invoke_signed,
    borsh0_10::try_from_slice_unchecked,
};

use crate::state::election_account_state::ElectionAccountState;
use crate::candidate_list_manager_account::generate_candidate_list_account;
use crate::pda_management::result_manager_account::generate_result_account;
use borsh::BorshSerialize;

pub fn add_election_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    start_date: NaiveDateTime,
    end_date: NaiveDateTime
) -> ProgramResult {
    //CONVERSIONE DELLE DATE IN STRINGHE
    let formatted_start_date = start_date.format("%Y-%m-%d %H:%M:%S").to_string();
    let formatted_end_date = end_date.format("%Y-%m-%d %H:%M:%S").to_string();

    let electione_name = name.clone();
    let election_name_for_result = name.clone();

    //CREA ITERATORE SU ACCOUNTS
    let account_info_iter = &mut accounts.iter();

    //RECUPERA ACCOUNT FORNITI DA ACCOUNT
    let initializer = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let election_pda_account = next_account_info(account_info_iter)?;
    //DERIVA PDA
    let (election_pda, election_bump_seed) = Pubkey::find_program_address(
        &[program_id.as_ref(), name.as_bytes().as_ref()],
         program_id
        );    
    
    //CALCOLA DIMENSIONE DELL'ACCOUNT
    let account_len: usize = 1 +
     (4 * name.len()) + 
     (4 * formatted_start_date.len()) +
     (4 * formatted_end_date.len()) +
    10000;

    //CALCOLA IL COSTO DI RENT
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    //CREA L'ACCOUNT 
    invoke_signed(
        &system_instruction::create_account(
            initializer.key, 
            election_pda_account.key, 
            rent_lamports,
            account_len.try_into().unwrap(), 
            program_id
        ), 
        &[initializer.clone(),election_pda_account.clone(),system_program.clone()], 
        &[&[program_id.as_ref(),name.as_bytes().as_ref(), &[election_bump_seed]]]
    )?;

    msg!("PDA Created: {}",election_pda);

    //INIZIALIZZA L'ACCOUNT 
    let is_election_created = initialize_election_account(election_pda_account, name, formatted_start_date, formatted_end_date);
       
    if is_election_created.is_ok(){

        let is_candidate_list_created = generate_candidate_list_account(program_id, accounts, electione_name);
    
        if is_candidate_list_created.is_ok() {
            
            let is_result_account_created = generate_result_account(program_id, accounts, election_name_for_result);

            if is_result_account_created.is_ok() {
                 Ok(())
            }
            else {
                return Err(ProgramError::IncorrectProgramId);
            }
        }
        else {
            return Err(ProgramError::IncorrectProgramId)
        }
    } else {
        return Err(ProgramError::AccountBorrowFailed)
    }

    
}

pub fn initialize_election_account(
    pda_account: &AccountInfo,
    name: String,
    start_date: String,
    end_date: String
) -> ProgramResult {
    msg!("Unpacking vote account");
    let mut account_data = try_from_slice_unchecked::<ElectionAccountState>(&pda_account.data.borrow()).unwrap();

    account_data.name = name;
    account_data.start_date = start_date;
    account_data.end_date = end_date;
    account_data.is_initialized = true;
    account_data.is_active = false;
    account_data.number_of_votes = 0;

    msg!("Serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("Account serialized");

    Ok(())
}

pub fn increment_vote_counter (
    election_pda_account: &AccountInfo
) -> ProgramResult {
    msg!("Unpacking vote account...");
    let mut account_data: ElectionAccountState = try_from_slice_unchecked::<ElectionAccountState>(&election_pda_account.data.borrow()).unwrap();

    account_data.number_of_votes +=1;

    msg!("Serializing account");
    account_data.serialize(&mut &mut election_pda_account.data.borrow_mut()[..])?;
    msg!("Vote account serialized");

    Ok(())
}

pub fn add_candidate_to_election(
    election_pda_account: &AccountInfo,
    candidate_address: Pubkey
) -> ProgramResult {
    msg!("Unpacking vote account...");
    let mut account_data: ElectionAccountState = try_from_slice_unchecked::<ElectionAccountState>(&election_pda_account.data.borrow()).unwrap();

    account_data.votes.insert(candidate_address, 0);

    msg!("Aggiunto Candidato all'Elezione");

    msg!("Serializing account");
    account_data.serialize(&mut &mut election_pda_account.data.borrow_mut()[..])?;
    msg!("Vote account serialized");

    Ok(())
}

pub fn add_vote(
    pda_account: &AccountInfo,
    candidate_address: Pubkey,

) -> ProgramResult {
    msg!("Unpacking vote account...");
    let mut account_data: ElectionAccountState = try_from_slice_unchecked::<ElectionAccountState>(&pda_account.data.borrow()).unwrap();

    msg!("Adding new vote");

    if let Some(value) = account_data.votes.get_mut(&candidate_address) {
        *value += 1;
    }
    account_data.number_of_votes +=1;

    msg!("Serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("Vote account serialized");

    Ok(())
}

pub fn get_percentage_of_votes (
    election_pda_account: &AccountInfo,
    candidate_pda_address: Pubkey
) -> Result<f32,ProgramError> {

    let account_data: ElectionAccountState = try_from_slice_unchecked::<ElectionAccountState>(&election_pda_account.data.borrow()).unwrap();
    
    let votes_for_candidate = account_data.votes.get(&candidate_pda_address).unwrap();
    let percentage = (100.0/account_data.number_of_votes as f32) * *votes_for_candidate as f32;

    return Ok(percentage);
}

//OTTIENE IL NUMERO TOTALE DI VOTI
pub fn get_number_of_votes (
    election_pda_account: &AccountInfo,
) -> Result<i64,ProgramError> {

    let account_data: ElectionAccountState = try_from_slice_unchecked::<ElectionAccountState>(&election_pda_account.data.borrow()).unwrap();
    
    let number_of_votes = account_data.number_of_votes;

    return Ok(number_of_votes)
}