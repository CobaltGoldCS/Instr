use app::App;
use crossterm::{
    cursor,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event::Key, KeyCode::Char},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    env,
    fs::File,
    io::{self, Read},
};

use ratatui::{
    prelude::{CrosstermBackend, Terminal},
    widgets::{Paragraph, Wrap},
    Frame,
};

pub mod app;
pub mod text_types;
pub mod tokenizer;

use tokenizer::tokenize_string;

fn main() -> Result<(), io::Error> {
    // setup terminal
    enable_raw_mode()?;

    execute!(
        io::stdout(),
        EnterAlternateScreen,
        EnableMouseCapture,
        cursor::MoveTo(0, 0)
    )?;
    let args: Vec<_> = env::args().collect();
    let path: &str = if args.len() > 1 {
        &args[1]
    } else {
        "instructions.inst"
    };

    let mut file = File::open(path).expect(&format!("{} does not exist", path));

    let mut file_string = String::new();
    file.read_to_string(&mut file_string)?;

    let mut tokens = tokenize_string(file_string).into_iter();

    let text_types = text_types::from_tokens(&mut tokens)?;
    let paragraph = Paragraph::new(text_types).wrap(Wrap { trim: true });
    let run = run(paragraph);

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

    run?;

    return Ok(());
}

fn run(widget: Paragraph) -> Result<(), io::Error> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App {
        scroll: (0, 0),
        should_quit: false,
    };

    loop {
        let widget = widget.clone().scroll(app.scroll);

        terminal.draw(|f| {
            display_frame(&mut app, f, vec![widget]);
        })?;
        update(&mut app)?;

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

fn display_frame(app: &mut App, frame: &mut Frame, widgets: Vec<Paragraph>) {
    for widget in widgets {
        let size = frame.size();
        frame.render_widget(widget.scroll(app.scroll), size);
    }
}

fn update(app: &mut App) -> Result<(), io::Error> {
    if !event::poll(std::time::Duration::from_millis(200))? {
        return Ok(());
    }
    if let Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Press {
            match key.code {
                Char('j') => app.scroll.0 = app.scroll.0 + 1,
                Char('k') => {
                    let mut y = app.scroll.0.wrapping_sub(1);
                    if y == u16::MAX {
                        y = 0;
                    }
                    app.scroll.0 = y;
                }
                Char('q') => app.should_quit = true,
                _ => {}
            }
        }
    }
    Ok(())
}
