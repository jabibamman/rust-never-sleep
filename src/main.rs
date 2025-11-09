#[cfg(windows)]
use log::error;
#[cfg(windows)]
use log::info;
use log::warn;
#[cfg(windows)]
use std::error::Error;
#[cfg(windows)]
use std::time::Duration;

#[cfg(windows)]
fn get_duration() -> Result<Duration, Box<dyn Error>> {
    use std::io::stdin;

    loop {
        let mut input = String::new();

        println!("Enter sleep duration in seconds or press Enter for default (14 minutes): ");

        if let Err(e) = stdin().read_line(&mut input) {
            error!("Failed to read input from stdin: {}", e);
            return Err(format!("Failed to read input from stdin: {}", e).into());
        }

        let trimmed = input.trim();

        if trimmed.is_empty() {
            warn!("No input provided, defaulting to 14 minutes");
            return Ok(Duration::from_secs(14 * 60));
        }

        match trimmed.parse::<u64>() {
            Ok(secs) if secs == 0 => {
                warn!("User entered 0 seconds, which is invalid (must be > 0)");
                println!("Duration must be greater than zero.");
                continue;
            }
            Ok(secs) => {
                info!("Sleeping for {} seconds", secs);
                return Ok(Duration::from_secs(secs));
            }
            Err(_) => {
                warn!("User entered invalid input for duration: '{}'", trimmed);
                println!("Invalid input, please enter a valid number.");
                continue;
            }
        }
    }
}

#[cfg(windows)]
fn let_me_sleep() -> Result<(), Box<dyn Error>> {
    use std::io;
    use std::thread::sleep;
    use winapi::um::winbase::SetThreadExecutionState;
    use winapi::um::winnt::{
        ES_AWAYMODE_REQUIRED, ES_CONTINUOUS, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED,
    };

    unsafe {
        let result = SetThreadExecutionState(
            ES_CONTINUOUS | ES_DISPLAY_REQUIRED | ES_SYSTEM_REQUIRED | ES_AWAYMODE_REQUIRED,
        );

        if result == 0 {
            return Err(io::Error::last_os_error().into());
        }
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
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    #[cfg(windows)]
    match let_me_sleep() {
        Ok(_) => (),
        Err(e) => eprintln!("Error during execution: {}", e),
    }

    #[cfg(not(windows))]
    warn!("This program is only useful on Windows");
}
