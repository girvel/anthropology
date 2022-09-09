use std::io;
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), io::Error> {
    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()));
    Ok(())
}
