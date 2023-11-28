#[cfg(windows)] extern crate winapi;
#[cfg(windows)] use log::info;

use log::warn;


#[cfg(windows)]
fn let_me_sleep() {
    use std::thread::sleep;

    use winapi::um::winnt::{ES_CONTINUOUS, ES_SYSTEM_REQUIRED, ES_AWAYMODE_REQUIRED};
    use winapi::um::winbase::SetThreadExecutionState;

    unsafe {
        SetThreadExecutionState(ES_CONTINUOUS);
    }

    let duration = std::time::Duration::from_secs(14 * 60);

    loop {
        sleep(duration);
        info!("I'm still alive");
    }

}

fn main() {
    env_logger::init();

    #[cfg(windows)]
    let_me_sleep();

    #[cfg(not(windows))]
    warn!("This program is only useful on Windows");
        
}

