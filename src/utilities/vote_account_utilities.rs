use chrono::{DateTime, Utc, Duration};
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError};



pub fn check_dates(
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>
) -> ProgramResult {

    //creo data di oggi e aggiungo 20 min
    let today_plus_20_min = Utc::now().checked_add_signed(Duration::minutes(20));

    if today_plus_20_min.unwrap() >= start_date {

        if start_date < end_date {
            return Ok(());
        } else {
            return Err(ProgramError::InvalidInstructionData);
        }
        
    } else {
        return Err(ProgramError::InvalidInstructionData);
    }
}