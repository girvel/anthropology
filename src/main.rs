use std::{io, thread, time::Duration};
use tui::{backend::CrosstermBackend, Terminal, Frame};
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use galgebra::matrix::Matrix;
use tui::backend::Backend;
use galgebra::vector::Vec2;
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::{Block, Borders};
use tui::widgets::canvas::Canvas;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    if let Err(err) = run_app(&mut terminal) {
        println!("Error during execution of the app: {:?}", err);
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut map = Matrix::filled(&String::from("."), Vec2(40, 15));
    map[Vec2(4, 3)] = String::from("@");

    terminal.draw(|f| ui(f, &map))?;

    thread::sleep(Duration::from_secs(5));
    Ok(())
}

fn ui<B: Backend>(f: &mut Frame<B>, map: &Matrix<String>) {
    let canvas = Canvas::default()
        .block(Block::default().borders(Borders::ALL).title("Aquarium"))
        .paint(move |ctx| {
            for y in 0..map.size().1 {
                for x in 0..map.size().0 {
                    let char = map[Vec2(x, y)].to_string();

                    let style = match char.as_str() {
                        "." => Style::default().fg(Color::LightYellow),
                        "@" => Style::default().fg(Color::LightBlue),
                        _ => panic!(),
                    };

                    ctx.print(x as f64, -(y as f64), Span::styled(char, style));
                }
            }
        })
        .x_bounds([0., f.size().width as f64 - 3.])
        .y_bounds([-(f.size().height as f64 - 3.), 0.]);

    f.render_widget(canvas, f.size());
}
