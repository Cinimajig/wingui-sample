use std::process::ExitCode;
use windows::Win32::UI::{WindowsAndMessaging::SW_SHOW, HiDpi::{SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2}};

#[path = "lib.rs"]
mod lib;

fn main() -> ExitCode {
    match run() {
        Ok(exit_code) => ExitCode::from(exit_code as u8),
        Err(err) => {
            eprintln!("{err}");
            ExitCode::from(err.code().0 as u8)
        },
    }

}

fn run() -> windows::core::Result<isize> {
    unsafe {
        SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2)?;
    }
    
    let mut window = lib::MainWindow::new()?;
    window.build(SW_SHOW)?;
    
    let exit_code = window.message_loop();
    Ok(exit_code)
}