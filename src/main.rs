use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders},
    Terminal
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::backend::Backend;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    if let Err(err) = run_app(&mut terminal) {
        println!("Error during execution of the app: {:?}", err);
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    terminal.draw(|f| {
        let size = f.size();
        let block = Block::default()
            .title("Hello world!")
            .borders(Borders::ALL);
        f.render_widget(block, size);
    })?;

    thread::sleep(Duration::from_millis(1000));
    Ok(())
}
