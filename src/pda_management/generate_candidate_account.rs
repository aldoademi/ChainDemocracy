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

use crate::{state::candidate_state::CandidateState, pda_management::generate_candidate_list_account::add_candidate_to_candidate_list};
use borsh::BorshSerialize;

pub fn add_candidate(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    first_name: String,
    last_name: String,
    election_name: String,
    seed: String
) -> ProgramResult {

    // Crea iteratore su accounts[]
    let account_info_iter = &mut accounts.iter();

    // Recupero account forniti da client
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let pda_candidate_list = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let candidate_first_name = first_name.clone();
    let candidate_last_name = last_name.clone();

    // Deriva PDA (Ottiene pubKey e bump)
    let (pda, bump_seed) = Pubkey::find_program_address(
        &[initializer.key.as_ref(), first_name.as_bytes().as_ref(),last_name.as_bytes().as_ref()],
         program_id
        );
    let(candidate_list_pda,c_bump_seed) = Pubkey::find_program_address(
        &[program_id.as_ref(),election_name.as_bytes().as_ref(),seed.as_bytes().as_ref()],
        program_id
    );
    

    // Calcola dimensione dell'account da creare
    let account_len: usize = 1 + (4 * first_name.len()) + (4 * last_name.len());

    // Calcola rent
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    //Crea account
    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id
        ),
        &[initializer.clone(),pda_account.clone(), system_program.clone()],
        &[&[initializer.key.as_ref(), first_name.as_bytes().as_ref(), last_name.as_bytes().as_ref(), &[bump_seed]]],
    )?;
    msg!("PDA Created: {}",pda);

    //inserisco dati nell'account generato
    let is_initialized = intialize_candidate_account(pda_account,first_name,last_name);

    if is_initialized.is_ok() {
        //Inserisce candidato nella lista candidati
        let is_candidate_in_list = add_candidate_to_candidate_list(program_id, pda_candidate_list, &pda, election_name, candidate_first_name, candidate_last_name, seed);

        if is_candidate_in_list.is_ok() {
            return Ok(())
        }
        else {
            return Err(ProgramError::AccountBorrowFailed) 
        }
    } else {
        return Err(ProgramError::AccountBorrowFailed)
    }

}

pub fn intialize_candidate_account (
    pda_account: &AccountInfo,
    first_name: String,
    last_name: String
) -> ProgramResult{

    msg!("Unpacking candidate account");
    let mut account_data = try_from_slice_unchecked::<CandidateState>(&pda_account.data.borrow()).unwrap();
    msg!("Borrowed account data");

    account_data.first_name = first_name;
    account_data.last_name = last_name;
    account_data.is_initialized = true;

    msg!("Serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("Account serialized");

    Ok(())
}