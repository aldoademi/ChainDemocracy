use borsh::BorshDeserialize;
use chrono:: NaiveDateTime;
use solana_program::program_error::ProgramError;

pub enum ChainDemocracyInstruction {
    AddElectionAccount{
        name: String,
        start_date: NaiveDateTime,
        end_date: NaiveDateTime
    },
    AddCandidateListAccount {
        election_name: String
    },
    UpdateElectionAccount {
        name: String,
    },
    AddCandidate {
        first_name: String,
        last_name: String,
        election_name: String,
        seed: String
    },
    AddVote {
        electoral_card_number: String,
        candidate_first_name: String,
        candidate_last_name: String,
        election_name: String,
        seed: String
    },
    CountingVotes {
        election_name: String,
    }
}

#[derive(BorshDeserialize)]
struct AddCandidatePayload {
    first_name: String,
    last_name: String,
    election_name: String,
    seed: String,
}
#[derive(BorshDeserialize)]
struct  AddElectionAccountPayload{
    name: String,
    start_date: String,
    end_date: String
}

#[derive(BorshDeserialize)]
struct  AddVotePayload{
    electoral_card_number: String,
    candidate_first_name: String,
    candidate_last_name: String,
    election_name: String,
    seed: String
}

#[derive(BorshDeserialize)]
struct  CountingVotesPayload{
    election_name: String,
}
impl ChainDemocracyInstruction {

    pub fn unpack(input: &[u8]) -> Result<Self,ProgramError> {

        let (&variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => {
                let payload = AddElectionAccountPayload::try_from_slice(rest).unwrap();
                let parsed_start_date = NaiveDateTime::parse_from_str(&payload.start_date, "%Y-%m-%dT%H:%M:%S").unwrap();
            
                let parsed_end_date = NaiveDateTime::parse_from_str(&payload.end_date, "%Y-%m-%dT%H:%M:%S").unwrap();

                Self::AddElectionAccount { name: payload.name, start_date: parsed_start_date, end_date: parsed_end_date }
            } 
            1 => {
                let payload = AddCandidatePayload::try_from_slice(rest).unwrap();
                Self::AddCandidate{
                    first_name: payload.first_name,
                    last_name: payload.last_name,
                    election_name: payload.election_name,
                    seed: payload.seed
                }
            }
            2 => {
                let payload = AddVotePayload::try_from_slice(rest).unwrap();
                Self::AddVote { 
                    electoral_card_number: payload.electoral_card_number,
                    candidate_first_name: payload.candidate_first_name,
                    candidate_last_name: payload.candidate_last_name,
                    election_name: payload.election_name,
                    seed: payload.seed
                 } 
            }
            3 => {
                let payload = CountingVotesPayload::try_from_slice(rest).unwrap();
                Self::CountingVotes {election_name: payload.election_name}
            }
            _=> return Err(ProgramError::InvalidInstructionData)
        })

    }
}