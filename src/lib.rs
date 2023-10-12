use solana_program::{
<<<<<<< HEAD
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{self, clock::Clock},
};
use std::io::Read;

// Definisci il programma
entrypoint!(process_instruction);

// Definisci l'ID del programma
solana_program::declare_id!("F9M23T99wqx9SNLFZsfnpWsc1G5wWFtTSQdew7xG4PwK");

// Definisci la struttura dati per il voto
#[derive(Debug, Default, PartialEq, Copy, Clone)]
struct Vote {
    voter: Pubkey,
    candidate: u32,
}

// Definisci la struttura dati per l'elezione
#[derive(Debug, Default, PartialEq, Clone)]
struct Election {
    votes: Vec<Vote>,
}

// Funzione principale del programma
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Analizza i dati di istruzione per determinare l'azione richiesta
    match instruction_data[0] {
        0 => initialize_election(program_id, accounts),
        1 => vote(program_id, accounts, instruction_data),
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

// Funzione per inizializzare l'elezione
fn initialize_election(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    // Assicurati di avere i permessi corretti
    let accounts_iter = &mut accounts.iter();
    let election_account = next_account_info(accounts_iter)?;

    // Verifica che l'account sia vuoto (non inizializzato)
    if election_account.data.borrow().iter().any(|&x| x != 0) {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // Crea una nuova elezione vuota
    let election_data = Election { votes: Vec::new() };

    // Salva i dati dell'elezione nell'account
    election_data.serialize(&mut &mut election_account.data.borrow_mut()[..])?;

    Ok(())
}

// Funzione per registrare un voto
fn vote(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    // Assicurati di avere i permessi corretti
    let mut accounts_iter = accounts.iter();
    let election_account = next_account_info(&mut accounts_iter)?;

    // Verifica che l'account sia inizializzato
    if election_account.data.borrow().iter().all(|&x| x == 0) {
        return Err(ProgramError::UninitializedAccount);
    }

    // Deserializza i dati dell'elezione
    let election_data = Election::deserialize(&election_account.data.borrow())?;

    // Analizza i dati dell'istruzione per ottenere i dettagli del voto
    let voter_pubkey = next_account_info(&accounts_iter)?.key;
    let candidate_id = instruction_data[1] as u32; // Supponiamo che il secondo byte contenga l'ID del candidato

    // Verifica che l'elettore non abbia già votato
    if election_data.votes.iter().any(|v| v.voter == *voter_pubkey) {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // Registra il voto
    let new_vote = Vote {
        voter: *voter_pubkey,
        candidate: candidate_id,
    };

    // Crea una nuova elezione con il voto aggiunto
    let mut new_election_data = Election {
        votes: election_data.votes.clone(),
    };
    new_election_data.votes.push(new_vote);

    // Salva i dati aggiornati nell'account
    new_election_data.serialize(&mut &mut election_account.data.borrow_mut()[..])?;

    // Ora puoi ottenere la chiave senza clonare
    let voter_pubkey = election_account.key;

    Ok(())
}

// Funzione di utilità per ottenere l'account successivo nell'iteratore
fn next_account_info<'a>(
    iter: &'a mut std::slice::Iter<'a, AccountInfo<'a>>,
) -> Result<&'a mut AccountInfo<'a>, ProgramError> {
    iter.next().ok_or(ProgramError::NotEnoughAccountKeys)
}

// Implementa la serializzazione e deserializzazione per le strutture dati
impl Vote {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.voter.to_bytes())?;
        writer.write_all(&self.candidate.to_le_bytes())?;
        Ok(())
    }

    fn deserialize(data: &[u8]) -> std::io::Result<Self> {
        let mut reader = std::io::Cursor::new(data);
        let mut voter_bytes = [0u8; 32];
        reader.read_exact(&mut voter_bytes)?;
        let voter = Pubkey::new_from_array(voter_bytes);

        let mut candidate_bytes = [0u8; 4];
        // Usa read al posto di read_exact
        reader.read(&mut candidate_bytes)?;
        let candidate = u32::from_le_bytes(candidate_bytes);

        Ok(Vote { voter, candidate })
    }
}

impl Election {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        for vote in &self.votes {
            vote.serialize(writer)?;
        }
        Ok(())
    }

    fn deserialize(data: &[u8]) -> std::io::Result<Self> {
        let mut reader = std::io::Cursor::new(data);
        let mut votes = Vec::new();
        while let Ok(vote) = Vote::deserialize(&reader.get_ref()[reader.position() as usize..]) {
            votes.push(vote);
        }
        Ok(Election { votes })
    }
}
=======
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

>>>>>>> 98f9f7d7f947f450fd91f31d5ef320412bcb3c02
