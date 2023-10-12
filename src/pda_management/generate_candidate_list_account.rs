use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    msg,
    account_info::{next_account_info, AccountInfo},
    system_instruction,
    program_error::ProgramError,
    sysvar::{rent::Rent, Sysvar},
    program::{invoke_signed},
    borsh::try_from_slice_unchecked, address_lookup_table::program,
};

use crate::{state::candidate_list_state::CandidateListState};
use borsh::{BorshDeserialize, BorshSerialize};

pub fn generate_candidate_list_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    first_name: String,
    last_name: String,
    name_vote_account: String
) -> ProgramResult{

    let account_info_iter = &mut accounts.iter();

    // Get accounts
    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    let seed = String::from("candidate-list");

     // Derive PDA and check that it matches client
     let (pda, bump_seed) = Pubkey::find_program_address(
        &[program_id.as_ref(), name_vote_account.as_bytes().as_ref(),seed.as_bytes().as_ref()],
         program_id
        );

    // Calculate account size required
    let account_len: usize = 1 + 10 * ((4 * first_name.len()) + (4 * last_name.len()) + 32);

    // Calculate rent required
    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    invoke_signed(
        &system_instruction::create_account(
            program_id,
            pda_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id
            ),
        &[initializer.clone(),pda_account.clone(),system_program.clone()],
        &[&[program_id.as_ref(),name_vote_account.as_bytes().as_ref(),seed.as_bytes().as_ref(), &[bump_seed]]]
        )?;
        msg!("PDA Created {}", pda);

        msg!("Unpacking candidate-list account");
        let mut account_data = try_from_slice_unchecked::<CandidateListState>(&pda_account.data.borrow()).unwrap();
        msg!("Borrowed account data");

        account_data.is_initialized = true;

        msg!("Serializing account");
        account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
        msg!("Account serialized");
    
        Ok(())


    
}