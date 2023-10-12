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
    borsh::try_from_slice_unchecked,
};

pub mod instruction;
use instruction::ChainDemocracyInstruction;
pub mod pda_management;
use pda_management::{generate_candidate_account, generate_vote_account};
pub mod state;
use state::candidate_state;
pub mod utilities;


entrypoint!(process_instruction);

pub fn process_instruction (
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data_instructions: &[u8]
) -> ProgramResult {

    let instruction = ChainDemocracyInstruction::unpack(data_instructions)?;

    match instruction {
        ChainDemocracyInstruction::AddVoteAccount { name, start_date, end_date } => {
           generate_vote_account::add_vote_account(program_id, accounts, name, start_date, end_date) ;
        }

        ChainDemocracyInstruction::UpdateVoteAccount { name } => {
            generate_vote_account::try_update(program_id, accounts, name);
        }

        ChainDemocracyInstruction::AddCandidate { first_name, last_name } => {
            generate_candidate_account::add_candidate(program_id, accounts, first_name, last_name);
        }
    }
    Ok(())
}
