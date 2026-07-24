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


/*
use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};

use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;
use ratatui::Terminal;
*/

use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::Duration;

use crate::engine::core::*;
use crate::engine::lexer::vfs::*;

pub struct Tui {
    logs: Vec<String>,
    path: String,
    time: Option<String>,
    trans: Option<Receiver<ParseEvent>>,
}

impl Tui {
    pub fn new() -> Self {
        Tui {
            logs: Vec::new(),
            path: String::new(),
            time: None,
            trans: None,
        }
    }
    
    pub fn load(&mut self, path: &str) {
        self.path = path.to_string();
        let (tx, rx) = mpsc::channel::<ParseEvent>();
        self.trans = Some(rx);
        let mut prolyxena = FsData::new_trans(path, tx.clone());

        thread::spawn(move || {
            prolyxena.load();
        });
        let _ = self.start_tui();
    }

    pub fn start_tui(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut indent = 0;
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        loop {
            self.parse_events(&mut indent);
            terminal.draw(|f| self.draw_ui(f))?;

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

    pub fn draw_ui(&mut self, frame: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(10),
                Constraint::Percentage(90)
            ])
            .split(frame.size());

        let time = match &self.time {
            Some(t) => t,
            None => "running...",
        };

        let sidebar_text = format!(" Pfad: {} \n Time: {} ", &self.path, time);
        let log_text = (&self.logs.join("\n")).to_string();

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

    pub fn parse_events(&mut self, indent: &mut usize) {
        if let Some(rx) = &self.trans {
            while let Ok(event) = rx.try_recv() {
                if let ParseEvent::Finished(time_str) = event {
                    self.time = Some(time_str);
                    continue;
                };
                let (is_start, name) = match event {
                    ParseEvent::Finished(time_str) => (true, "Finished"),

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
                    self.logs.push(format!("{} Starte {}", indent_string, name));
                    *indent += 1;
                } else {
                    self.logs.push(format!(" {} Scließe {}", indent_string, name));
                }
            }
        }
    }
}
