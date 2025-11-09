# Rust Never Sleep 🦀

_A lightweight Rust utility to keep Windows awake._

---

## Overview

**Rust Never Sleep** is a small command-line program written in Rust that prevents Windows from automatically entering sleep mode.  
It interacts directly with the Windows API through the [`winapi`](https://crates.io/crates/winapi) crate, using `SetThreadExecutionState` to tell the system to stay active.

---

## Features

- Keeps the system and display awake for a given duration.
- Accepts user input in seconds.
- Defaults to **14 minutes** when no input is provided.
- Handles invalid input (non-numeric or zero) gracefully.
- Uses structured logging (`info`, `warn`, `error`) via `log` + `env_logger`.
- Simple to build, simple to run — no dependencies beyond Rust itself.

---

## Prerequisites

- 🪟 **Windows OS**
- 🦀 **Rust toolchain** ([Install here](https://www.rust-lang.org/tools/install))
- **Cargo** (included with Rust)

---

## Installation & Build

1. Clone the repository:

   ```bash
   git clone https://github.com/jabibamman/rust-never-sleep.git
   cd rust-never-sleep
   ```

2. Build the project in release mode:

   ```bash
   cargo build --release
   ```

3. The compiled binary will be located at:
   ```
   target/release/rust-never-sleep.exe
   ```

---

## Usage

Run the executable directly or through a terminal:

```powershell
./rust-never-sleep.exe
```

You will be prompted:

```
Enter sleep duration in seconds or press Enter for default (14 minutes):
```

**Examples:**

- Press **Enter** → uses the default duration of 14 minutes.
- Enter `600` → keeps the system awake for 10 minutes.
- Enter `0` or any invalid text → prints an error and asks again.

To stop the program, simply close the terminal window.

---

## Logging

`rust-never-sleep` uses the [`log`](https://crates.io/crates/log) crate with [`env_logger`](https://crates.io/crates/env_logger) for runtime logging.

By default, logs are shown at the `info` level.  
You can change this by setting the environment variable `RUST_LOG`:

```powershell
$env:RUST_LOG="debug"
./rust-never-sleep.exe
```

Available levels: `error`, `warn`, `info`, `debug`, `trace`.

---

## Technical Details

- **Language:** Rust
- **Target:** `x86_64-pc-windows-gnu`
- **Windows API used:** `SetThreadExecutionState`
  - Flags: `ES_CONTINUOUS`, `ES_DISPLAY_REQUIRED`, `ES_SYSTEM_REQUIRED`, `ES_AWAYMODE_REQUIRED`
- **Error Handling:**
  - Input errors are logged and retried.
  - Windows API failures are logged using `std::io::Error::last_os_error()`.

---

## Notes

- Designed exclusively for **Windows** systems.
- Ensure the use of this utility aligns with your organization’s policies.
- While running, the program prevents automatic sleep and display timeout events.

---

## Contributing

Contributions are welcome!  
If you’d like to improve the code, fix issues, or add features, please open a Pull Request.

---

## License

MIT License © 2025 [James Abib](https://github.com/jabibamman)
