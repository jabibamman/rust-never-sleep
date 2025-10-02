#[cfg(windows)] extern crate winapi;
#[cfg(windows)] use std::error::Error;
#[cfg(windows)] use std::time::Duration;

use log::warn;

#[cfg(windows)]
fn get_duration() -> Result<Duration, Box<dyn Error>> {
    use std::io::stdin;

    let mut duration  = None;
    while duration.is_none() {
        let mut input = String::new(); // here there is the example that we don't need nor want a mutable variable 
        //because we are not going to change it after the initialization.
        
        println!("Enter sleep duration in seconds or press Enter for default (14 minutes): ");
        if stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input, please try again.");
            continue;
        }

        let trimmed = input.trim();

        if trimmed.is_empty() {
            eprintln!("No input, defaulting to 14 minutes");
            duration = Some(Duration::from_secs(14 * 60));
            break;
        }

        let trimmed_parse: Option<Result<u64, _>> = Some(trimmed.parse::<u64>());

        match trimmed_parse {
            None => {
                eprintln!("Invalid input, please enter a valid number.");
                continue;
            },
            Some(Err(_)) => {
                eprintln!("Invalid input, please enter a valid number.");
                continue;
            },
            Some(Ok(secs)) if secs == 0 => {
                eprintln!("Duration must be greater than zero.");
                continue;
            },
            Some(Ok(secs)) => {
                return Ok(Duration::from_secs(secs));
            },

        }
    }    

    Err("Error while getting the duration".into())
} 



#[cfg(windows)]
fn let_me_sleep() {
    use std::thread::sleep;
    use std::time::Duration;

    use winapi::um::winnt::{ES_CONTINUOUS, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED, ES_AWAYMODE_REQUIRED};
    use winapi::um::winbase::SetThreadExecutionState;

    unsafe {
        SetThreadExecutionState(ES_CONTINUOUS | ES_DISPLAY_REQUIRED | ES_SYSTEM_REQUIRED | ES_AWAYMODE_REQUIRED);
    }

    let mut duration: Duration;
    let duration_result = get_duration();
    match duration_result {
        Ok(duration_trimmed) =>  {
            duration = duration_trimmed;
        }
        Err(e) => {
            eprintln!("Error during retrieving the duration : {}", e);
        }
    }
    
    loop {
        sleep(duration); // here we have a duration which is a Option<Duration>. To fix that we have to unwrap it 
        // but we also don't want to use the unwrap function cause of the rust best practices. 
        // the purpose here is to guarantee the duration is not None or sum.
    }

}

fn main() {
    env_logger::init();

    #[cfg(windows)]
    let_me_sleep();

    #[cfg(not(windows))]
    warn!("This program is only useful on Windows");
        
}

