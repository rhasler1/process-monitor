use std::io::{stdout, Stdout};
use anyhow::Result;
use ratatui::{backend::CrosstermBackend, Terminal};
use crossterm::{execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen,
        disable_raw_mode, enable_raw_mode
    },
    event::{DisableMouseCapture, EnableMouseCapture}
};


/// Setup terminal with alternate screen & mouse capture enabled.
///
/// On error, attempts to restore terminal and returns error msg.
pub fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    // Cleanup on error
    if let Err(e) = execute!(
        stdout(),
        EnterAlternateScreen,
        EnableMouseCapture
    ) {
        let _ = disable_raw_mode();
        return Err(e.into())
    }

    // Cleanup on error
    if let Err(e) = terminal.clear() {
        // Full cleanup
        let _ = restore_terminal();
        return Err(e.into())
    }

    Ok(terminal)
}

/// Restore terminal by leaving alternate
/// screen and disabling mouse capture.
pub fn restore_terminal() -> Result<()> {
    disable_raw_mode()?;
    execute!(
        stdout(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    
    Ok(())
}


