use std::{io, thread, time::Duration};
use std::rc::Rc;
use tui::{backend::CrosstermBackend, Terminal, Frame};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use galgebra::matrix::Matrix;
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
    let mut map = Matrix::new([
        [String::from(" "), String::from(" "), String::from(" ")],
        [String::from(" "), String::from("a"), String::from(" ")],
        [String::from(" "), String::from(" "), String::from(" ")],
        [String::from(" "), String::from(" "), String::from(" ")],
        [String::from(" "), String::from(" "), String::from(" ")],
        [String::from(" "), String::from(" "), String::from(" ")],
    ]);

    terminal.draw(|f| ui(f, map))?;

    thread::sleep(Duration::from_millis(1000));
    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, map: Matrix<String>) {
    let canvas = Canvas::default()
        .paint(move |ctx| {
            for y in 0..map.size().1 {
                for x in 0..map.size().0 {
                    ctx.print(0., 0., Span::styled(
                        &map[Vec2(x, y)],
                        Style::default().fg(Color::Black).bg(Color::White)
                    ));
                }
            }
        });

    f.render_widget(canvas, f.size());
}
