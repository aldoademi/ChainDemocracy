// Importa le librerie di Solana
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{self, clock::Clock},
};

// Definisci il programma
entrypoint!(process_instruction);

// Definisci l'ID del programma
solana_program::declare_id!("YourProgramID");

// Definisci la struttura dati per il voto
struct Vote {
    voter: Pubkey,
    candidate: u32,
}

// Definisci la struttura dati per l'elezione
struct Election {
    votes: Vec<Vote>,
    // Altri campi per l'elezione possono essere aggiunti secondo necessità
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
    let mut election_data = Election { votes: Vec::new() };

    // Salva i dati dell'elezione nell'account
    election_data.serialize(&mut &mut election_account.data.borrow_mut()[..])?;

    Ok(())
}

// Funzione per registrare un voto
fn vote(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
    // Assicurati di avere i permessi corretti
    let accounts_iter = &mut accounts.iter();
    let election_account = next_account_info(accounts_iter)?;

    // Verifica che l'account sia inizializzato
    if election_account.data.borrow().iter().all(|&x| x == 0) {
        return Err(ProgramError::UninitializedAccount);
    }

    // Deserializza i dati dell'elezione
    let mut election_data = Election::deserialize(&election_account.data.borrow())?;

    // Analizza i dati dell'istruzione per ottenere i dettagli del voto
    let voter_pubkey = next_account_info(accounts_iter)?.key;
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
    election_data.votes.push(new_vote);

    // Salva i dati aggiornati nell'account
    election_data.serialize(&mut &mut election_account.data.borrow_mut()[..])?;

    Ok(())
}

// Funzione di utilità per ottenere l'account successivo nell'iteratore
fn next_account_info<'a, 'b>(
    iter: &'a mut std::slice::Iter<'b, AccountInfo>,
) -> Result<&'b AccountInfo<'b>, ProgramError> {
    iter.next().ok_or(ProgramError::NotEnoughAccountKeys)
}

// Implementa la serializzazione e deserializzazione per le strutture dati
impl Vote {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&self.voter.to_bytes())?;
        writer.write_all(&self.candidate.to_le_bytes())?;
        Ok(())
    }

    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let mut voter_bytes = [0u8; 32];
        reader.read_exact(&mut voter_bytes)?;
        let voter = Pubkey::new_from_array(voter_bytes);

        let mut candidate_bytes = [0u8; 4];
        reader.read_exact(&mut candidate_bytes)?;
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

    fn deserialize<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        let mut votes = Vec::new();
        while let Ok(vote) = Vote::deserialize(reader) {
            votes.push(vote);
        }
        Ok(Election { votes })
    }
}
