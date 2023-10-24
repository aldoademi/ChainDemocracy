use chrono::{NaiveDateTime, Utc, Duration};
use solana_program::{entrypoint::ProgramResult, program_error::ProgramError};



pub fn check_dates(
    start_date: NaiveDateTime,
    end_date: NaiveDateTime
) -> ProgramResult {

    //creo data di oggi e aggiungo 20 min
    let now = Utc::now().naive_utc();
    let today_plus_20_min = now + Duration::minutes(20);

    if today_plus_20_min >= start_date {

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
    let today = Utc::now().naive_utc();

    //Verifico di essere all'interno del tempo di inizio e fine
    if(today >= start_date){
        // Verifico di essere ancora in tempo nell'eseguire le operazioni
        if(end_date > today){
            return Ok(());
        }else{
            return Err(ProgramError::InvalidInstructionData);
        }
    }else{
        return Err(ProgramError::InvalidInstructionData);
    }
}

pub fn check_time_registration(
    start_date: NaiveDateTime,
    end_date: NaiveDateTime
) -> ProgramResult{
    
    //Creo data di oggi
    let today = Utc::now().naive_utc();

    // Verifico che le elezioni non siano ancora iniziate
    if(start_date > today){
        return Ok(());
    }else{
        return Err(ProgramError::InvalidInstructionData);
    }
}