use std::{io, thread, time::Duration};
use tui::{backend::CrosstermBackend, Terminal, Frame};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::backend::Backend;
use galgebra::vector::Vec2;
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::canvas::Canvas;

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
    terminal.draw(|f| ui(f))?;

    thread::sleep(Duration::from_millis(1000));
    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>) {
    let canvas = Canvas::default()
        .paint(|ctx| {
            ctx.print(0., 0., Span::styled(
                format!("Look, a vector: {}", Vec2(-1, 1) + Vec2(2, 2)),
                Style::default().fg(Color::Black).bg(Color::White)
            ));
        });

    f.render_widget(canvas, f.size());
}
