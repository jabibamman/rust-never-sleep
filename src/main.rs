#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
use std::error::Error;
#[cfg(windows)]
use std::time::Duration;

use log::warn;

#[cfg(windows)]
fn get_duration() -> Result<Duration, Box<dyn Error>> {
    use std::io::stdin;

    loop {
        let mut input = String::new();

        println!("Enter sleep duration in seconds or press Enter for default (14 minutes): ");
        if stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input, please try again.");
            continue;
        }

        let trimmed = input.trim();

        if trimmed.is_empty() {
            eprintln!("No input, defaulting to 14 minutes");
            return Ok(Duration::from_secs(14 * 60));
            break;
        }

        let trimmed_parse = match trimmed.parse::<u64>() {
            Ok(secs) => Some(Ok(secs)),
            Err(e) => Some(Err(e)),
        };

        match trimmed_parse {
            None => {
                eprintln!("Invalid input, please enter a valid number.");
                continue;
            }
            Some(Err(_)) => {
                eprintln!("Invalid input, please enter a valid number.");
                continue;
            }
            Some(Ok(secs)) if secs == 0 => {
                eprintln!("Duration must be greater than zero.");
                continue;
            }
            Some(Ok(secs)) => {
                return Ok(Duration::from_secs(secs));
            }
        }
    }

    Err("Error while getting the duration".into())
}

#[cfg(windows)]
fn let_me_sleep() -> Result<(), Box<dyn std::error::Error>> {
    use std::thread::sleep;
    use winapi::um::winbase::SetThreadExecutionState;
    use winapi::um::winnt::{
        ES_AWAYMODE_REQUIRED, ES_CONTINUOUS, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED,
    };

    unsafe {
        SetThreadExecutionState(
            ES_CONTINUOUS | ES_DISPLAY_REQUIRED | ES_SYSTEM_REQUIRED | ES_AWAYMODE_REQUIRED,
        );
    }

    let duration = match get_duration() {
        Ok(d) => d,
        Err(e) => {
            eprintln!("Error getting duration: {}", e);
            return Err(e);
        }
    };

    loop {
        sleep(duration);
    }
}

fn main() {
    env_logger::init();

    #[cfg(windows)]
    match let_me_sleep() {
        Ok(_) => (),
        Err(e) => eprintln!("Error during execution: {}", e),
    }

    #[cfg(not(windows))]
    warn!("This program is only useful on Windows");
}
