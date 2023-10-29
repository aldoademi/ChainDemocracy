use std::collections::HashMap;

use solana_program::{
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    account_info::{next_account_info, AccountInfo},
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
    program::invoke_signed,
    borsh0_10::try_from_slice_unchecked
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
    let account_len: usize = 10000;

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
    accounts: &[AccountInfo]
) -> ProgramResult {
    
    let account_info_iter = &mut accounts.iter();

    let _initializer = next_account_info(account_info_iter)?;
    let election_pda_account = next_account_info(account_info_iter)?;
    let candidate_list_pda_account = next_account_info(account_info_iter)?;
    let result_pda_account = next_account_info(account_info_iter)?;
    let _system_program = next_account_info(account_info_iter)?;

    //RECUPERA L'HASHMAP DEI CANDIDATI
    let candidate_list = retrieve_candidate_list(candidate_list_pda_account)?;
    
    //OTTIENE NUMERO TOTALE VOTI E LO INSERISCE IN RESULT
    let total_number_of_votes = get_number_of_votes(election_pda_account)?;
    let _ = add_number_of_votes(result_pda_account, total_number_of_votes);
    //AGGIUNGE E STAMPA I RISULTATI
    let _ = add_and_show_result(candidate_list, election_pda_account, result_pda_account);
    
    Ok(())

}

pub fn add_and_show_result (
    candidate_list: HashMap<String,Pubkey>,
    election_pda_account: &AccountInfo,
    result_pda_account: &AccountInfo
) -> ProgramResult {
    let mut not_sorted_hash_map: HashMap<String,f32> = HashMap::new();

    for (candidate_info, candidate_pda_address) in candidate_list {
        let percentage_for_candidate = get_percentage_of_votes(election_pda_account, candidate_pda_address)?;
        not_sorted_hash_map.insert(candidate_info, percentage_for_candidate);  
    }
    return sort_and_add_results(result_pda_account, not_sorted_hash_map);
}

pub fn add_number_of_votes (
    result_pda_account: &AccountInfo,
    number_of_votes: i64
) -> ProgramResult {

    let mut account_data: ResultState = try_from_slice_unchecked::<ResultState>(&result_pda_account.data.borrow()).unwrap();

    account_data.number_of_votes = number_of_votes;
    account_data.serialize(&mut &mut result_pda_account.data.borrow_mut()[..])?;
    
    Ok(())
}


pub fn sort_and_add_results (
    result_pda_account: &AccountInfo,
    not_sorted_hash_map: HashMap<String,f32>,
) -> ProgramResult {

    let mut account_data: ResultState = try_from_slice_unchecked::<ResultState>(&result_pda_account.data.borrow()).unwrap();

    let mut tuple_vec: Vec<_> = not_sorted_hash_map.into_iter().collect();

    // ORDINA IL VETTORE IN ORDINE DECRESCENTE
    tuple_vec.sort_by(|a, b| b.1.total_cmp(&a.1));

    for (k,v) in tuple_vec.clone()  {
        msg!("Il candidato {} ha ricevuto il {}% dei voti", k,v);
        account_data.results.insert(k,v);
    }

    msg!("Voti totali: {}",account_data.number_of_votes);

  
    account_data.serialize(&mut &mut result_pda_account.data.borrow_mut()[..])?;


    Ok(())

}
