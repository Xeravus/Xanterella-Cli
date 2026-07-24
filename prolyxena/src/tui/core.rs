use std::io;
use crossterm::{
    event::{
        self, Event, KeyCode
    },
    execute,
    terminal::{
        disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen
    },
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{
        Alignment, Constraint, Direction, Layout
    },
    widgets::{
        Block, Borders, Paragraph
    },
    Frame,
    Terminal,
};
use std::{
    sync::mpsc, thread, time::Duration
};
use std::sync::mpsc::Receiver;
use crate::engine::core::*;

pub struct Tui {
    logs: Vec<String>,
    path: String,
    time: i32,
}

/*
pub impl Tui {
    pub fn new() -> Self {
        Tui {
            logs: Vec::new(),
            path: String::new(),
            time: 0,
        }
    }
*/

pub fn start_tui(path: &str, time: i32) -> Result<(), Box<dyn std::error::Error>> {
    let mut state = Tui {
        logs: Vec::new(),
        path: path.to_string(),
        time,
    };
    let mut indent = 0;
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        //
    });

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        parse_events(&rx, &mut state, &mut indent);
        terminal.draw(|f| draw_ui(f, &state))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    break;
                }
            }
        }
    }
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

pub fn draw_ui(frame: &mut Frame, state: &Tui) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(10),
            Constraint::Percentage(90)
        ])
        .split(frame.size());

    // let sidebar_text = format!(" Pfad: {} \n Time: {} ", &self.path, &self.time);
    let sidebar_text = format!(" Pfad: ~/xanterella \n Time: 0.05s ");
    let log_text = state.logs.join("\n");

    let sidebar_block = Block::default()
        .title(" Prolyxena Output ")
        .borders(Borders::ALL);
    let main_block = Block::default()
        .title(" Prolyxena Parsegraph ")
        .borders(Borders::ALL);

    let sidebar = Paragraph::new(sidebar_text).block(sidebar_block);
    let main = Paragraph::new(log_text).block(main_block);

    frame.render_widget(sidebar, chunks[0]);
    frame.render_widget(main, chunks[1]);
}

pub fn parse_events(rx: &Receiver<ParseEvent>, state: &mut Tui, indent: &mut usize) {
    while let Ok(event) = rx.try_recv() {
        let (is_start, name) = match event {
            ParseEvent::StartAttrSet => (true, "Attribut Set"),
            ParseEvent::EndAttrSet => (false, "Attribut Set"),

            ParseEvent::StartList => (true, "Liste"),
            ParseEvent::EndList => (false, "Liste"),

            ParseEvent::StartLetIn => (true, "LetIn"),
            ParseEvent::EndLetIn => (false, "LetIn"),

            ParseEvent::StartLambda => (true, "Lambda"),
            ParseEvent::EndLambda => (false, "Lambda"),

            ParseEvent::StartWith => (true, "With"),
            ParseEvent::EndWith => (false, "With"),

            ParseEvent::StartString => (true, "String"),
            ParseEvent::EndString => (false, "String"),

            ParseEvent::StartPath => (true, "Path"),
            ParseEvent::EndPath => (false, "Path"),

            ParseEvent::StartNumber => (true, "Number"),
            ParseEvent::EndNumber => (false, "Number"),

            ParseEvent::StartExpression => (true, "Expression"),
            ParseEvent::EndExpression => (false, "Expression"),

            ParseEvent::StartOperator => (true, "Operator"),
            ParseEvent::EndOperator => (false, "Operator"),

            ParseEvent::StartIdentifier => (true, "Identifier"),
            ParseEvent::EndIdentifier => (false, "Identifier"),

            ParseEvent::StartWhitespace => (true, "Whitespace"),
            ParseEvent::EndWhitespace => (false, "Whitespace"),

            ParseEvent::StartValue => (true, "Value"),
            ParseEvent::EndValue => (false, "Value"),

            ParseEvent::StartGroup => (true, "Group"),
            ParseEvent::EndGroup => (false, "Group"),

            ParseEvent::StartAntiquotation => (true, "Antiquotation"),
            ParseEvent::EndAntiquotation => (false, "Antiquotation"),

            ParseEvent::StartIndentedString => (true, "Intented String"),
            ParseEvent::EndIndentedString => (false, "Intented String"),
        };

        if !is_start && *indent > 0 {
            *indent -= 1;
        }

        let indent_string = "  ".repeat(*indent);

        if is_start {
            state.logs.push(format!("{} Starte {}", indent_string, name));
            *indent += 1;
        } else {
            state.logs.push(format!(" {} Schlue0e {}", indent_string, name));
        }
    }
}
