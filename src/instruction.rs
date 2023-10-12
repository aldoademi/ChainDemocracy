use borsh::BorshDeserialize;
use chrono::{DateTime, Utc, NaiveDate, NaiveDateTime, TimeZone, format::Fixed, FixedOffset};
use solana_program::program_error::ProgramError;

pub enum ChainDemocracyInstruction {
    AddVoteAccount{
        name: String,
        start_date: NaiveDateTime,
        end_date: NaiveDateTime
    },
    AddCandidateListAccount {
        election_name: String
    },
    UpdateVoteAccount {
        name: String,
    },
    AddCandidate {
        first_name: String,
        last_name: String,
        election_name: String,
        seed: String
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
struct  AddVoteAccountPayload{
    name: String,
    start_date: String,
    end_date: String
}

#[derive(BorshDeserialize)]
struct  UpdateVoteAccountPayload{
    name: String,
}
#[derive(BorshDeserialize)]
struct  AddCandidateListAccountPayload{
    election_name: String,
}


impl ChainDemocracyInstruction {

    pub fn unpack(input: &[u8]) -> Result<Self,ProgramError> {

        let (&variant, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => {
                let payload = AddVoteAccountPayload::try_from_slice(rest).unwrap();
                let parsed_start_date = NaiveDateTime::parse_from_str(&payload.start_date, "%Y-%m-%dT%H:%M:%S").unwrap();
            
                let parsed_end_date = NaiveDateTime::parse_from_str(&payload.end_date, "%Y-%m-%dT%H:%M:%S").unwrap();

                Self::AddVoteAccount { name: payload.name, start_date: parsed_start_date, end_date: parsed_end_date }
            } 
            1 => {
                let payload = AddCandidateListAccountPayload::try_from_slice(rest).unwrap();
                Self::AddCandidateListAccount { election_name: payload.election_name }

            }
            2 => {
                let payload = AddCandidatePayload::try_from_slice(rest).unwrap();
                Self::AddCandidate{
                    first_name: payload.first_name,
                    last_name: payload.last_name,
                    election_name: payload.election_name,
                    seed: payload.seed
                }
            }
            3 => {
                let payload = UpdateVoteAccountPayload::try_from_slice(rest).unwrap();
                Self::UpdateVoteAccount { name: payload.name }
            }
            _=> return Err(ProgramError::InvalidInstructionData)
        })

    }
}