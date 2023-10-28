use borsh::BorshSerialize;
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

use crate::{
    state::voter_account_state::VoterAccountState,
     pda_management::{candidate_list_manager_account::retrieve_candidate_account,
         election_manager_account::add_vote}
};

pub fn add_voter_account_and_vote (
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    electoral_card_number: String,
    candidate_first_name: String,
    candidate_last_name: String,
    election_name: String,
    seed: String
) -> ProgramResult {

    //Crea iteratore su accounts[]
    let account_info_iter = &mut accounts.iter();
    //Recupera account forniti da client
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let candidate_list_pda_account = next_account_info(account_info_iter)?;
    let election_pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    //Deriva PDA
    let (pda, bump_seed) = Pubkey::find_program_address(
        &[program_id.as_ref(), election_name.as_bytes().as_ref(), electoral_card_number.as_bytes().as_ref()],
         program_id
        );

    let (candidate_pda, _candidate_bump_seed) = Pubkey::find_program_address(
        &[program_id.as_ref(), election_name.as_bytes().as_ref(), seed.as_bytes().as_ref()],
            program_id
        );

    let (election_pda, _election_bump_seed) = Pubkey::find_program_address(
         &[program_id.as_ref(), election_name.as_bytes().as_ref()],
            program_id
        );
    
    //Verifiche che i PDA derivati hanno lo stesso indirizzo degli account forniti dal client
    if candidate_pda != *candidate_list_pda_account.key || election_pda != *election_pda_account.key{
        msg!("Invalid seed for account");
        return Err(ProgramError::InvalidSeeds)
    }
    
    //Calcola dimensione dell'account
    let account_len: usize = 4 * electoral_card_number.len() + 32;

    //Calcola costo di rent
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
        &[&[program_id.as_ref(),election_name.as_bytes().as_ref(),electoral_card_number.as_bytes().as_ref(), &[bump_seed]]],
    )?;

    msg!("PDA Created: {}",pda);

    //inizializza account
    // let is_initialized = initialize_voter_account( pda_account, electoral_card_number);

    // if is_initialized.is_ok() {
    //     //recupera il candidato in candidate list, se lo trova (Ok) aggiunge voto nell'account elezione
    //     match  retrieve_candidate_account( candidate_list_pda_account, candidate_first_name, candidate_last_name) {
    //         Ok(candidate_address) => {
    //             let vote = add_vote(election_pda_account, candidate_address, &pda);
    //         }
    //         Err(error)=> {
    //             msg!("Error, invalid candidate");
    //             return Err(ProgramError::InvalidAccountData)
    
    //         }
    //     }
    // } else {
    //     msg!("Error in voter account");
    //     return Err(ProgramError::AccountAlreadyInitialized)
    // }


    
    //recupera il candidato in candidate list, se lo trova (Ok) aggiunge voto nell'account elezione
    match  retrieve_candidate_account( candidate_list_pda_account, candidate_first_name.clone(), candidate_last_name.clone()) {
        Ok(candidate_address) => {

            let is_voter_initialized = initialize_voter_account(pda_account, electoral_card_number, candidate_address);

            if is_voter_initialized.is_ok() {
                
                let _ = add_vote(election_pda_account, candidate_address);
                msg!("Hai votato {} {}", candidate_first_name, candidate_last_name);
                return Ok(());
                
            } else {
                return Err(ProgramError::InvalidSeeds);
            }

        }
        Err(error)=> {
            msg!("Error, invalid candidate {}",error);
            return Err(ProgramError::InvalidAccountData)
    
        }
    }
}


pub fn initialize_voter_account (
    pda_account: &AccountInfo,
    electoral_card_number: String,
    candidate_address: Pubkey
) ->ProgramResult {

    msg!("Unpacking voter account");
    let mut account_data = try_from_slice_unchecked::<VoterAccountState>(&pda_account.data.borrow()).unwrap();
    msg!("Borrowed account data");
    
    account_data.election_card_number = electoral_card_number;
    account_data.voted = candidate_address;


    msg!("Serializing account");
    account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
    msg!("Account serialized");
    
    Ok(())

    
}