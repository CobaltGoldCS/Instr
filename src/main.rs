use app::App;
use crossterm::{
    cursor,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event::Key, KeyCode::Char},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::{Paragraph, Wrap},
    Frame,
};

pub mod app;
pub mod text_types;
pub mod tokenizer;

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;
    execute!(
        io::stdout(),
        EnterAlternateScreen,
        EnableMouseCapture,
        cursor::MoveTo(0, 0)
    )?;

    let run = run();

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

    run?;

    return Ok(());
}

fn run() -> Result<(), io::Error> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;
    let text_types = text_types::convert_text_types("instructions.inst")?;

    let paragraph = Paragraph::new(text_types).wrap(Wrap { trim: true });

    let mut app = App {
        scroll: (0, 0),
        should_quit: false,
    };

    loop {
        let widget = paragraph.clone().scroll(app.scroll);

        terminal.draw(|f| {
            display_frame(&mut app, f, vec![widget]).expect("Renders to screen");
        })?;
        update(&mut app)?;

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

fn display_frame(
    app: &mut App,
    frame: &mut Frame,
    widgets: Vec<Paragraph>,
) -> Result<(), io::Error> {
    for widget in widgets {
        let size = frame.size();
        frame.render_widget(widget.scroll(app.scroll), size);
    }
    Ok(())
}

fn update(app: &mut App) -> Result<(), io::Error> {
    if !event::poll(std::time::Duration::from_millis(16))? {
        return Ok(());
    }
    if let Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Press {
            match key.code {
                Char('j') => {
                    let y = app.scroll.0 + 1;
                    app.scroll = (y, app.scroll.1);
                }
                Char('k') => {
                    let mut y = app.scroll.0.wrapping_sub(1);
                    if y == u16::MAX {
                        y = 0;
                    }
                    app.scroll = (y, app.scroll.1);
                }
                Char('q') => app.should_quit = true,
                _ => {}
            }
        }
    }
    Ok(())
}
