use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    account_info::AccountInfo,
    program_error::ProgramError,
};

pub mod instruction;
use instruction::ChainDemocracyInstruction;
pub mod pda_management;
use pda_management::{generate_candidate_account, generate_election_account, generate_candidate_list_account, vote_manager_accout::add_voter_account_and_vote};
pub mod state;
pub mod utilities;


entrypoint!(process_instruction);

pub fn process_instruction (
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data_instructions: &[u8]
) -> ProgramResult {

    let instruction = ChainDemocracyInstruction::unpack(data_instructions)?;

    match instruction {
        ChainDemocracyInstruction::AddElectionAccount { name, start_date, end_date } => {
           generate_election_account::add_election_account(program_id, accounts, name, start_date, end_date) ;
        }
        ChainDemocracyInstruction::AddCandidateListAccount { election_name } => {
            generate_candidate_list_account::generate_candidate_list_account(program_id, accounts, election_name);
        }

        ChainDemocracyInstruction::AddVote { electoral_card_number,candidate_first_name, candidate_last_name ,election_name,seed} => {
            add_voter_account_and_vote(program_id, accounts, electoral_card_number, candidate_first_name, candidate_last_name, election_name, seed);
            
        }

        ChainDemocracyInstruction::AddCandidate { first_name, last_name, election_name, seed } => {
            generate_candidate_account::add_candidate(program_id, accounts, first_name, last_name, election_name, seed);
        }
        _=> return  Err(ProgramError::InvalidAccountData)
    }
    Ok(())
}

