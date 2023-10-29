use solana_program::{
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    account_info::AccountInfo,
    program_error::ProgramError, msg,
};

pub mod instruction;
use instruction::ChainDemocracyInstruction;
pub mod pda_management;
use pda_management::{candidate_manager_account, election_manager_account, candidate_list_manager_account, voter_manager_accout::add_voter_account_and_vote, result_manager_account::counting_votes};
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
        //CREA ACCOUNT ELEZIONE, LISTA CANDIDATI E RISULTATI
        ChainDemocracyInstruction::AddElectionAccount { name, start_date, end_date } => {
           let _ = election_manager_account::add_election_account(program_id, accounts, name, start_date, end_date);
        }
        //CREA ACCOUNT CANDIDATO
        ChainDemocracyInstruction::AddCandidate { first_name, last_name, election_name, seed } => {
            let _ = candidate_manager_account::add_candidate(program_id, accounts, first_name, last_name, election_name, seed);
        }
        //CREA ACCOUNT VOTANTE E REGISTRA IL VOTO NELL'ACCOUNT ELEZIONE 
        ChainDemocracyInstruction::AddVote { electoral_card_number,candidate_first_name, candidate_last_name ,election_name,seed} => {
            let _ = add_voter_account_and_vote(program_id, accounts, electoral_card_number, candidate_first_name, candidate_last_name, election_name, seed);  
        }
        //POPOLA L'ACCOUNT RISULTATI CON I RISULTATI DEI VOTI 
        ChainDemocracyInstruction::CountingVotes { election_name } => {
            msg!("Risultati delle {}",election_name);
            let _ = counting_votes(accounts);
        }
        _=> return  Err(ProgramError::InvalidAccountData)
    }
    Ok(())
}


