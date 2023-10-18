use chrono::{NaiveDateTime, Utc, Duration};
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError};



pub fn check_dates(
    start_date: NaiveDateTime,
    end_date: NaiveDateTime
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

pub fn check_time_election(
    start_date: NaiveDateTime,
    end_date: NaiveDateTime
) -> ProgramResult{
    
    //Creo data di oggi
    let today = Utc::now();

    //Verifico di essere all'interno del tempo di inizio e fine
    if(today.unwrap() >= start_date){
        // Verifico di essere ancora in tempo nell'eseguire le operazioni
        if(end_date > today.unwrap()){
            return Ok(());
        }else{
            return Err(ProgramError::InvalidInstructionData);
        }
    }else{
        return Err(ProgramError::InvalidInstructionData);
    }
}