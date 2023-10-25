use std::collections::HashMap;

use solana_program::{
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    account_info::{next_account_info, AccountInfo},
    system_instruction,
    program_error::ProgramError,
    sysvar::{rent::Rent, Sysvar},
    program::invoke_signed,
    borsh::try_from_slice_unchecked, address_lookup_table::program,
};

use crate::state::candidate_list_state::CandidateListState;
use borsh::BorshSerialize;

pub fn generate_candidate_list_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    election_name: String
) -> ProgramResult{

    //Crea iteratore su accounts[]
    let account_info_iter = &mut accounts.iter();

    // Recupero account forniti da client
    let initializer = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let _election_pda_account = next_account_info(account_info_iter)?;
    let candidate_list_pda_account = next_account_info(account_info_iter)?;

    let seed = String::from("candidate-list");

     // Deriva PDA
     let (candidate_list_pda, candidate_list_bump_seed) = Pubkey::find_program_address(
        &[program_id.as_ref(), election_name.as_bytes().as_ref(),seed.as_bytes().as_ref()],
         program_id
        );

    
    let account_len: usize = 1000;

    // Calcola il costo di rent
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    //Crea l'account
    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            candidate_list_pda_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id
            ),
        &[initializer.clone(),candidate_list_pda_account.clone(),system_program.clone()],
        &[&[program_id.as_ref(),election_name.as_bytes().as_ref(),seed.as_bytes().as_ref(), &[candidate_list_bump_seed]]]
        )?;

        msg!("PDA Created {}", candidate_list_pda);

        msg!("Unpacking candidate-list account");
        let mut account_data = try_from_slice_unchecked::<CandidateListState>(&candidate_list_pda_account.data.borrow()).unwrap();
        msg!("Borrowed account data");

        account_data.is_initialized = true;

        msg!("Serializing account");
        account_data.serialize(&mut &mut candidate_list_pda_account.data.borrow_mut()[..])?;
        msg!("Account serialized");
        msg!("Candidate list created for {}", election_name);
    
        Ok(())

}

pub fn add_candidate_to_candidate_list(
    program_id: &Pubkey,
    pda_account: &AccountInfo,
    address_candidate: &Pubkey,
    election_name: String,
    candidate_first_name: String,
    candidate_last_name: String,
    seed: String
) -> ProgramResult {
    //controlla che l'owner sia il programma
    if pda_account.owner != program_id{
        return Err(ProgramError::IllegalOwner);
    }

    //deriva PDA
    let(pda, _bump_seed) = Pubkey::find_program_address(
        &[program_id.as_ref(),election_name.as_bytes().as_ref(),seed.as_bytes().as_ref()],
         program_id
        );

    //Controlla che l'account derivato abbia la stesso indirizzo di quello fornito come parametro della funzione
    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA");
        return Err(ProgramError::InvalidSeeds)
    }
    msg!("Retrieve candidate list account: {}",pda);
    //ottiene dati dell'account
    let mut account_data: CandidateListState = try_from_slice_unchecked::<CandidateListState>(&pda_account.data.borrow()).unwrap();

    if !account_data.is_initialized {
        msg!("Account not initialized");
        return Err(ProgramError::InvalidAccountData)
    }

    msg!("Updating candidate list...");
    //inserisce candidato
    account_data.candidate.insert(format!("{}{}", candidate_first_name,candidate_last_name), *address_candidate);

    let info = account_data.candidate.get(&format!("{}{}", candidate_first_name,candidate_last_name)).unwrap();

    msg!("New candidate {} with account {}", &format!("{}{}", candidate_first_name,candidate_last_name), info);

    for(key, value) in account_data.candidate.clone(){
        msg!("L'account di {} con chiave: {}",key,value);
    }

    msg!("Serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("Account serialized");

    Ok(())
}

pub fn retrieve_candidate_account<'a>(
    pda_account: &AccountInfo,
    candidate_first_name: String,
    candidate_last_name: String,
) -> Result<Pubkey,ProgramError>{

    let account_data: CandidateListState = try_from_slice_unchecked::<CandidateListState>(&pda_account.data.borrow()).unwrap();

    //--------------------------------------------------------------------
    for (key,value) in account_data.candidate.clone()  {
        msg!("Account di {} con chiave {}",key,value)
    }

    msg!("Searching for {}{}",candidate_first_name,candidate_last_name);

    //--------------------------------------------------------------------

    let candidate_address = account_data.candidate.get(&format!("{}{}",candidate_first_name,candidate_last_name)).unwrap();
   
    let candidate_address_copy = *candidate_address;
    return Ok(candidate_address_copy)
}

pub fn retrieve_candidate_list (
    candidate_list_pda_account: &AccountInfo,
) -> Result<HashMap<String,Pubkey>, ProgramError> {

    let account_data = try_from_slice_unchecked::<CandidateListState>(&candidate_list_pda_account.data.borrow()).unwrap();
    let candidate_list_copy = account_data.candidate.clone();

    return Ok(candidate_list_copy)
}