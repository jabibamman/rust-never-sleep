#[cfg(windows)] extern crate winapi;
#[cfg(windows)] use log::info;

use log::warn;


#[cfg(windows)]
fn let_me_sleep() {
    use std::io::{stdin, Stdin};
    use std::thread::sleep;
    use std::time::Duration;

    use winapi::um::winnt::{ES_CONTINUOUS, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED, ES_AWAYMODE_REQUIRED};
    use winapi::um::winbase::SetThreadExecutionState;

    unsafe {
        SetThreadExecutionState(ES_CONTINUOUS | ES_DISPLAY_REQUIRED | ES_SYSTEM_REQUIRED | ES_AWAYMODE_REQUIRED);
    }


    let mut duration: Option<Duration> = None;

        while duration.is_none() {
        let mut input = String::new();
        
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
                duration = Some(Duration::from_secs(secs));
            },
        }

        if duration.is_none() {
            eprintln!("Failed to parse duration, please try again.");
        }
    }
    
    loop {
        sleep(duration);
        debug!("I'm still alive");
    }

}

fn main() {
    env_logger::init();

    #[cfg(windows)]
    let_me_sleep();

    #[cfg(not(windows))]
    warn!("This program is only useful on Windows");
        
}

