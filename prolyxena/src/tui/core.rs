use std::borrow::Cow;
use std::io;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode};
use ratatui::Frame;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph};

use crate::engine::core::*;
use crate::engine::lexer::vfs::*;

#[derive(Debug)]
pub struct Tui {
    logs: Vec<ParseTask>,
    path: String,
    time: Option<String>,
    time_rx: Option<Receiver<String>>,
    trans: Option<Receiver<ParseEvent>>,
    last_update: Instant,
    num_of_pars: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    Running,
    Finished,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseTask {
    pub name: Cow<'static, str>,
    pub indent: usize,
    pub status: TaskStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TaskBool {
    False,
    True,
    Keep,
}

impl Tui {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Tui {
            logs: Vec::new(),
            path: String::new(),
            time: None,
            time_rx: None,
            trans: None,
            last_update: Instant::now(),
            num_of_pars: 0,
        }
    }

    pub fn load(&mut self, path: &str) {
        self.path = path.to_string();
        let (tx, rx) = mpsc::channel::<ParseEvent>();
        let (time_tx, time_rx) = mpsc::channel::<String>();
        self.trans = Some(rx);
        self.time_rx = Some(time_rx);
        let mut prolyxena = FsData::new_trans(path, tx.clone());

        #[cfg(not(test))]
        thread::spawn(move || {
            prolyxena.load();
            let _ = time_tx.send(prolyxena.get_time());
        });
        #[cfg(not(test))]
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

            if event::poll(Duration::from_millis(2))? && let Event::Key(key) = event::read()? && let KeyCode::Char('q') = key.code {
                break;
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
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)])
            .split(frame.size());

        let time = match &self.time {
            Some(t) => t,
            None => "running...",
        };

        let sidebar_text =
            format!(" Path: {} \n Time: {} \n Number of Actions: {}", self.path, time, self.num_of_pars);

        let sidebar_block = Block::default().title(" Prolyxena Output ").borders(Borders::ALL);
        let main_block = Block::default().title(" Prolyxena Parsegraph ").borders(Borders::ALL);

        let sidebar = Paragraph::new(sidebar_text).block(sidebar_block);

        let items: Vec<ListItem> = self
            .logs
            .iter()
            .map(|task| {
                let indent_str = "  ".repeat(task.indent);
                let text = match task.status {
                    TaskStatus::Running => format!("{} [ ] Start {}", indent_str, task.name),
                    TaskStatus::Finished => format!("{} [x] Finished {}", indent_str, task.name),
                };
                ListItem::new(text)
            })
            .collect();
        let list = List::new(items).block(main_block);
        let mut list_state = ListState::default();
        if !self.logs.is_empty() {
            list_state.select(Some(self.logs.len() - 1));
        }

        frame.render_widget(sidebar, chunks[0]);
        frame.render_stateful_widget(list, chunks[1], &mut list_state);
    }

    pub fn parse_events(&mut self, indent: &mut usize) {
        if let Some(time_rx) = &self.time_rx && let Ok(time_str) = time_rx.try_recv() {
            self.time = Some(time_str);
        }

        if let Some(rx) = &self.trans && self.last_update.elapsed() >= Duration::from_millis(2) && let Ok(event) = rx.try_recv() {
            self.last_update = Instant::now();
            if let ParseEvent::Finished(time_str) = &event {
                self.time = Some(time_str.clone());
                return;
            };
            let (taskbool, name): (TaskBool, Cow<'static, str>) = match event {
                ParseEvent::Finished(_) => (TaskBool::Keep, "Finished".into()),

                ParseEvent::StartGen => (TaskBool::True, "Generating Tree".into()),
                ParseEvent::EndGen => (TaskBool::False, "Generating Tree".into()),

                ParseEvent::StartGettingFiles => (TaskBool::True, "Getting Files".into()),
                ParseEvent::EndGettingFiles => (TaskBool::False, "Getting Files".into()),

                ParseEvent::StartParsingFile(file) => {
                    (TaskBool::True, format!("Generating AST: {}", file).into())
                }
                ParseEvent::EndParsingFile(file) => {
                    (TaskBool::Keep, format!("Generating AST: {}", file).into())
                }

                ParseEvent::StartAttrSet => (TaskBool::True, "Parsing Attribut Set".into()),
                ParseEvent::EndAttrSet => (TaskBool::False, "Parsing Attribut Set".into()),

                ParseEvent::StartList => (TaskBool::True, "Parsing List".into()),
                ParseEvent::EndList => (TaskBool::False, "Parsing List".into()),

                ParseEvent::StartLetIn => (TaskBool::True, "Parsing Let-In".into()),
                ParseEvent::EndLetIn => (TaskBool::False, "Parsing Let-In".into()),

                ParseEvent::StartLambda => (TaskBool::True, "Parsing Lambda".into()),
                ParseEvent::EndLambda => (TaskBool::False, "Parsing Lambda".into()),

                ParseEvent::StartWith => (TaskBool::True, "Parsing With".into()),
                ParseEvent::EndWith => (TaskBool::False, "Parsing With".into()),

                ParseEvent::StartString => (TaskBool::True, "Parsing String".into()),
                ParseEvent::EndString => (TaskBool::False, "Parsing String".into()),

                ParseEvent::StartPath => (TaskBool::True, "Parsing Path".into()),
                ParseEvent::EndPath => (TaskBool::False, "Parsing Path".into()),

                ParseEvent::StartNumber => (TaskBool::True, "Parsing Number".into()),
                ParseEvent::EndNumber => (TaskBool::False, "Parsing Number".into()),

                ParseEvent::StartExpression => (TaskBool::True, "Parsing Expression".into()),
                ParseEvent::EndExpression => (TaskBool::False, "Parsing Expression".into()),

                ParseEvent::StartOperator => (TaskBool::True, "Parsing Operator".into()),
                ParseEvent::EndOperator => (TaskBool::False, "Parsing Operator".into()),

                ParseEvent::StartIdentifier => (TaskBool::True, "Parsing Identifier".into()),
                ParseEvent::EndIdentifier => (TaskBool::False, "Parsing Identifier".into()),

                ParseEvent::StartWhitespace => (TaskBool::True, "Skipping Whitespace".into()),
                ParseEvent::EndWhitespace => (TaskBool::False, "Skipping Whitespace".into()),

                ParseEvent::StartValue => (TaskBool::True, "Parsing Value".into()),
                ParseEvent::EndValue => (TaskBool::False, "Parsing Value".into()),

                ParseEvent::StartGroup => (TaskBool::True, "Parsing Group".into()),
                ParseEvent::EndGroup => (TaskBool::False, "Parsing Group".into()),

                ParseEvent::StartAntiquotation => (TaskBool::True, "Parsing Antiquotation".into()),
                ParseEvent::EndAntiquotation => (TaskBool::False, "Parsing Antiquotation".into()),

                ParseEvent::StartIndentedString => (TaskBool::True, "Parsing Intented String".into()),
                ParseEvent::EndIndentedString => (TaskBool::False, "Parsing Intented String".into()),
            };

            if matches!(taskbool, TaskBool::False) && *indent > 0 {
                *indent -= 1;
            }

            match taskbool {
                TaskBool::True => {
                    if name != "Finished" {
                        self.logs.push(ParseTask { name, indent: *indent, status: TaskStatus::Running });
                        self.num_of_pars += 1;
                        *indent += 1;
                    } else {
                        self.logs.push(ParseTask {
                            name: "Parse erfolgreich beendet".into(),
                            indent: 0,
                            status: TaskStatus::Finished,
                        });
                    }
                }
                TaskBool::False => {
                    if let Some(index) = self.logs.iter().rposition(|t| t.status == TaskStatus::Running) {
                        self.logs.remove(index);
                    }
                }
                TaskBool::Keep => {
                    if let Some(index) = self.logs.iter().rposition(|t| t.name == name) {
                        self.logs.remove(index);
                    }
                    *indent -= 1;
                    self.logs.push(ParseTask { name, indent: *indent, status: TaskStatus::Finished });
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "core_test.rs"]
mod tests;
