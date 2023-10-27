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

use borsh::BorshSerialize;
use crate::{candidate_list_manager_account::retrieve_candidate_list, state::result_state::ResultState};

use super::election_manager_account::{get_percentage_of_votes, get_number_of_votes};

pub fn generate_result_account (
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    election_name: String,
) -> ProgramResult {

    let seed = String::from("result");

    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let _election_pda_account = next_account_info(account_info_iter)?;
    let _candidate_list_pda_account = next_account_info(account_info_iter)?;
    let result_pda_account = next_account_info(account_info_iter)?;

    let (result_pda, result_bump_seed) = Pubkey::find_program_address(
        &[program_id.as_ref(), election_name.as_bytes().as_ref(), seed.as_bytes().as_ref()],
         program_id
        );    
    let account_len: usize = 1000;

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    invoke_signed(
        &system_instruction::create_account(
            initializer.key, 
            result_pda_account.key, 
            rent_lamports,
            account_len.try_into().unwrap(), 
            program_id
        ), 
        &[initializer.clone(),result_pda_account.clone(),system_program.clone()], 
        &[&[program_id.as_ref(),election_name.as_bytes().as_ref(),seed.as_bytes().as_ref(), &[result_bump_seed]]]
    )?;

    msg!("PDA Created: {}",result_pda);

    Ok(())
}


pub fn counting_votes(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    election_name: String,
    candidate_list_seed: String,
    result_seed: String
) -> ProgramResult {
    
    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let election_pda_account = next_account_info(account_info_iter)?;
    let candidate_list_pda_account = next_account_info(account_info_iter)?;
    let result_pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;


    let (election_pda, election_bump_seed) = Pubkey::find_program_address(
        &[program_id.as_ref(), election_name.as_bytes().as_ref()],
         program_id
        );
    let (candidate_list_pda, candidate_list_bump_seed) = Pubkey::find_program_address(
        &[program_id.as_ref(), election_name.as_bytes().as_ref(), candidate_list_seed.as_bytes().as_ref()],
        program_id
        );
    let (result_pda, result_bump_seed) = Pubkey::find_program_address(
        &[program_id.as_ref(), election_name.as_bytes().as_ref(), result_seed.as_bytes().as_ref()],
        program_id
        );

    if election_pda != *election_pda_account.key ||
       candidate_list_pda != *candidate_list_pda_account.key ||
       result_pda != *result_pda_account.key {

        return Err(ProgramError::InvalidSeeds)
    }

    //recupero l'hashmap contenente le info sui candidato
    let candidate_list = retrieve_candidate_list(candidate_list_pda_account)?;

    for (candidate_info, candidate_pda_address) in candidate_list {
        //calcola la percentuale dei voti ricevuti
        let percentage_for_candidate = get_percentage_of_votes(election_pda_account, candidate_pda_address)?;
        let _ = add_result(result_pda_account, candidate_info, percentage_for_candidate);
    }

    let number_of_votes = get_number_of_votes(election_pda_account)?;
    add_number_of_votes(result_pda_account, number_of_votes);

    show_result(result_pda_account);

    
    Ok(())
}

pub fn add_result (
    result_pda_account: &AccountInfo,
    candidate_info: String,
    percentage_received: f64,
) -> ProgramResult {

    msg!("Unpacking vote account...");
    let mut account_data: ResultState = try_from_slice_unchecked::<ResultState>(&result_pda_account.data.borrow()).unwrap();

    account_data.results.insert(candidate_info,  percentage_received);

    msg!("Serializing account");
    account_data.serialize(&mut &mut result_pda_account.data.borrow_mut()[..])?;
    msg!("Account serialized");

    Ok(())
}

pub fn add_number_of_votes (
    result_pda_account: &AccountInfo,
    number_of_votes: i64
) -> ProgramResult {

    msg!("Unpacking vote account...");
    let mut account_data: ResultState = try_from_slice_unchecked::<ResultState>(&result_pda_account.data.borrow()).unwrap();

    account_data.number_of_votes = number_of_votes;

    msg!("Serializing account");
    account_data.serialize(&mut &mut result_pda_account.data.borrow_mut()[..])?;
    msg!("Account serialized");

    Ok(())
}

pub fn show_result(
    result_pda_account: &AccountInfo,
) -> ProgramResult {
   
    let mut account_data: ResultState = try_from_slice_unchecked::<ResultState>(&result_pda_account.data.borrow()).unwrap();

    for (key,value) in account_data.results.clone()  {
        msg!("Il Candidato {} ha ricevuto {}% dei voto", key,value)
    }

    msg!("Numero voti totali: {}", account_data.number_of_votes);

    

    Ok(())
}
