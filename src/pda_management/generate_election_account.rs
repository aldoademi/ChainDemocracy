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
    borsh::try_from_slice_unchecked,
};

use crate::state::election_account_state::ElectionAccountState;
use borsh::BorshSerialize;


pub fn add_election_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    name: String,
    start_date: NaiveDateTime,
    end_date: NaiveDateTime
) -> ProgramResult {

    //Converte le date in stringhe
    let formatted_start_date = start_date.format("%Y-%m-%d %H:%M:%S").to_string();
    let formatted_end_date = end_date.format("%Y-%m-%d %H:%M:%S").to_string();

    //Crea Iteratore su accounts[]
    let account_info_iter = &mut accounts.iter();

    //Prende gli account forniti dal client
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    //Deriva PDA
    let (pda, bump_seed) = Pubkey::find_program_address(
        &[program_id.as_ref(), name.as_bytes().as_ref()],
         program_id
        );
    
    //Calcola dimensione dell'account
    let account_len: usize = 1 +
     (4 * name.len()) + 
     (4 * formatted_start_date.len()) +
     (4 * formatted_end_date.len()) +
     6 * (32 + 32 * 10);

    //calcola il costo di rent
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    //Crea l'account
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

    //inizializza l'account
    let is_election_created = initialize_election_account(pda_account, name, formatted_start_date, formatted_end_date);
       
   
    if is_election_created.is_ok(){
        return Ok(())
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

    msg!("Serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("Account serialized");

    Ok(())

}


pub fn add_vote(
    pda_account: &AccountInfo,
    candidate_address: Pubkey,
    voter_address: &Pubkey

) -> ProgramResult {

    
    msg!("Unpacking vote account...");
    let mut account_data: ElectionAccountState = try_from_slice_unchecked::<ElectionAccountState>(&pda_account.data.borrow()).unwrap();


    msg!("Adding new vote");
    // ottiene una referenza mutabile al valore associato alla chiave.
    let entry = account_data.votes.entry(candidate_address).or_insert(Vec::new());
    // Aggiungi il valore al vettore.
    entry.push(*voter_address);


    // Stampa tutti i valori nel HashMap
    for (key, values) in &account_data.votes {
        msg!("Candidato: {:?}", key);
        msg!("Votato da: :");
        for value in values {
            msg!("  {:?}", value);
        }
    }
    msg!("Serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("Vote account serialized");

    Ok(())
}