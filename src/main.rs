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
    prelude::{Constraint, CrosstermBackend, Layout, Terminal},
    style::{Color, Modifier, Style},
    symbols::line::THICK,
    widgets::{block::Position, Block, Borders, LineGauge, Paragraph, Wrap},
    Frame,
};

pub mod app;
pub mod text_types;
pub mod tokenizer;

use tokenizer::tokenize_string;

fn main() -> Result<(), io::Error> {
    let args: Vec<_> = env::args().collect();
    let path: &str = if args.len() > 1 {
        &args[1]
    } else {
        "instructions.inst"
    };

    // setup terminal
    enable_raw_mode()?;

    execute!(
      io::stdout(),
      EnterAlternateScreen,
      EnableMouseCapture,
      cursor::MoveTo(0, 0)
     )?;

    let mut file = File::open(path).expect(&format!("{} does not exist", path));

    let mut file_string = String::new();
    file.read_to_string(&mut file_string)?;

    let mut tokens = tokenize_string(&file_string).into_iter();

    let text_types = text_types::from_tokens(&file_string[..1], &mut tokens)?;

    let app = App {
        scroll: (0, 0),
        total_lines: text_types.len() as u16,
        should_quit: false,
    };

    let paragraph = Paragraph::new(text_types).wrap(Wrap { trim: true });
    let run = run(app, paragraph);

    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

    run?;

    return Ok(());
}

fn run(mut app: App, widget: Paragraph) -> Result<(), io::Error> {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend)?;

    loop {
        let widget = widget.clone().scroll(app.scroll);

        terminal.draw(|f| {
            display_instruction_frame(&mut app, f, widget);
        })?;
        update(&mut app)?;

        if app.should_quit {
            break;
        }
    }
    Ok(())
}

fn display_instruction_frame(app: &mut App, frame: &mut Frame, widget: Paragraph) {
    let block = Block::new()
        .title("Instructions (q to quit, j to move down, k to move up)")
        .title_style(Style::new().add_modifier(Modifier::SLOW_BLINK))
        .borders(Borders::ALL)
        .title_position(Position::Top);

    let layout = Layout::default()
        .constraints(vec![Constraint::Percentage(95), Constraint::Percentage(5)])
        .split(frame.size());

    // Calculate the progress of the instructions as the (current scroll + height) / total lines
    let ratio = ((app.scroll.0 as f64 + layout[0].height as f64 - 1.0) / (app.total_lines as f64))
        .clamp(0.0, 1.0);

    let line_style = if ratio >= 1.0 {
        Style::default().fg(Color::LightGreen).bg(Color::Black)
    } else {
        Style::default().fg(Color::White).bg(Color::Black)
    };

    let progress_bar = LineGauge::default()
        .line_set(THICK)
        .gauge_style(line_style)
        .ratio(ratio);

    frame.render_widget(widget.block(block.clone()).scroll(app.scroll), layout[0]);
    frame.render_widget(progress_bar, layout[1]);
}

fn update(app: &mut App) -> Result<(), io::Error> {
    if !event::poll(std::time::Duration::from_millis(200))? {
        return Ok(());
    }
    if let Key(key) = event::read()? {
        if key.kind == event::KeyEventKind::Press {
            match key.code {
                Char('j') => {
                    app.scroll.0 = (app.scroll.0 + 1).clamp(0, app.total_lines);
                }
                Char('k') => {
                    app.scroll.0 =
                        (app.scroll.0 as isize - 1).clamp(0, app.total_lines as i32) as u16;
                }
                Char('q') => app.should_quit = true,
                _ => {}
            }
        }
    }
    Ok(())
}
