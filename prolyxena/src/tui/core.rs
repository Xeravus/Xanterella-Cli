use std::borrow::Cow;
use std::env::var;
use std::io;
use std::sync::mpsc::{self, Receiver};
#[allow(unused_imports)]
use std::thread;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode};
use ratatui::Frame;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
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
    sort: bool,
    debug: bool,
    expand: bool,
    flatten: bool,
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
            sort: false,
            debug: false,
            expand: false,
            flatten: false,
        }
    }

    pub fn set_sort(&mut self, sort: bool) {
        self.sort = sort;
    }

    pub fn set_expand(&mut self, expand: bool) {
        self.expand = expand;
    }

    pub fn set_flatten(&mut self, flatten: bool) {
        self.flatten = flatten;
    }

    pub fn set_debug(&mut self, debug: bool) {
        self.debug = debug;
    }

    pub fn load(&mut self, path: &str) {
        self.path = path.to_string();
        let (tx, rx) = mpsc::channel::<ParseEvent>();
        #[allow(unused)]
        let (time_tx, time_rx) = mpsc::channel::<String>();
        self.trans = Some(rx);
        self.time_rx = Some(time_rx);
        let mut prolyxena = FsData::new_trans(path, tx.clone());
        prolyxena.set_sort(self.sort);
        prolyxena.set_expand(self.expand);
        prolyxena.set_flatten(self.flatten);

        #[cfg(not(test))]
        thread::spawn(move || {
            prolyxena.load();
            let _ = time_tx.send(prolyxena.get_time());
        });
        if var("PROLYXENA_TEST").is_err() {
            #[cfg(not(test))]
            let _ = self.start_tui();
        }
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

            if event::poll(Duration::from_millis(2))?
                && let Event::Key(key) = event::read()?
                && let KeyCode::Char('q') = key.code
            {
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

        let sidebar_text = format!(
            " Path: {} \n Time: {} \n Number of Actions: {} / {}",
            self.path,
            time,
            self.num_of_pars,
            self.logs.len()
        );

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
        if let Some(time_rx) = &self.time_rx
            && let Ok(time_str) = time_rx.try_recv()
        {
            self.time = Some(time_str);
        }

        if let Some(rx) = &self.trans
            && self.last_update.elapsed() >= Duration::from_millis(2)
            && let Ok(event) = rx.try_recv()
        {
            self.last_update = Instant::now();
            if let ParseEvent::Finished(time_str) = &event {
                self.time = Some(time_str.clone());
                return;
            };
            let (taskbool, name): (TaskBool, Cow<'static, str>) = match event {
                ParseEvent::Finished(_) => (TaskBool::Keep, "Finished".into()),

                ParseEvent::StartGen => (TaskBool::True, "Generating Tree".into()),
                ParseEvent::EndGen => (TaskBool::Keep, "Generating Tree".into()),

                ParseEvent::StartGettingFiles => (TaskBool::True, "Getting Files".into()),
                ParseEvent::EndGettingFiles => (TaskBool::False, "Getting Files".into()),

                ParseEvent::StartParsingFile(file) => (TaskBool::True, format!("Generating AST: {}", file).into()),
                ParseEvent::EndParsingFile(file) => (TaskBool::Keep, format!("Generating AST: {}", file).into()),

                ParseEvent::StartSortingFile(file) => (TaskBool::True, format!("Sorting AST: {}", file).into()),
                ParseEvent::EndSortingFile(file) => (TaskBool::False, format!("Sorting AST: {}", file).into()),

                ParseEvent::StartExpandingFile(file) => (TaskBool::True, format!("Expanding AST: {}", file).into()),
                ParseEvent::EndExpandingFile(file) => (TaskBool::False, format!("Expanding AST: {}", file).into()),

                ParseEvent::StartFlatteningFile(file) => (TaskBool::True, format!("Flattening AST: {}", file).into()),
                ParseEvent::EndFlatteningFile(file) => (TaskBool::False, format!("Flattening AST: {}", file).into()),

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
                    if !self.debug {
                        if let Some(index) = self
                            .logs
                            .iter()
                            .rposition(|t| t.status == TaskStatus::Running || t.status == TaskStatus::Finished)
                        {
                            self.logs.remove(index);
                        }
                    } else {
                        self.logs.push(ParseTask { name, indent: *indent, status: TaskStatus::Finished });
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
mod tests {
    use super::*;

    #[test]
    fn test_tui_core_new() {
        let data = Tui::new();

        assert!(data.logs.is_empty());
        assert!(data.path.is_empty());
        assert!(data.time.is_none());
        assert!(data.time_rx.is_none());
        assert!(data.trans.is_none());
        assert_eq!(data.num_of_pars, 0);
    }

    #[test]
    fn test_tui_core_channels_time() {
        let mut tui = Tui::new();
        let mut indent = 0;
        let (tx, rx) = mpsc::channel();
        tui.time_rx = Some(rx);

        tx.send("0.052s".to_string()).unwrap();

        tui.parse_events(&mut indent);

        assert_eq!(tui.time, Some("0.052s".to_string()));
    }

    #[test]
    fn test_tui_core_parse_events_clock() {
        let mut tui = Tui::new();
        let mut indent = 0;
        let (tx, rx) = mpsc::channel();
        tui.trans = Some(rx);

        tx.send(ParseEvent::StartAttrSet).unwrap();

        tui.last_update = Instant::now() - Duration::from_millis(10);

        tui.parse_events(&mut indent);

        assert_eq!(tui.logs.len(), 1);
        assert_eq!(tui.logs[0].name, "Parsing Attribut Set");
        assert_eq!(tui.logs[0].status, TaskStatus::Running);
        assert_eq!(tui.logs[0].indent, 0);

        assert_eq!(indent, 1);
    }

    #[test]
    fn test_tui_core_parse_events_remove() {
        let mut tui = Tui::new();
        let mut indent = 0;
        let (tx, rx) = mpsc::channel();
        tui.trans = Some(rx);

        tx.send(ParseEvent::StartList).unwrap();
        tui.last_update = Instant::now() - Duration::from_millis(10);
        tui.parse_events(&mut indent);

        assert_eq!(tui.logs.len(), 1);
        assert_eq!(indent, 1);

        tx.send(ParseEvent::EndList).unwrap();
        tui.last_update = Instant::now() - Duration::from_millis(10);
        tui.parse_events(&mut indent);

        assert_eq!(tui.logs.len(), 0);
        assert_eq!(indent, 0);
    }

    #[test]
    fn test_tui_core_parse_events_keeps_finished_tasks() {
        let mut tui = Tui::new();
        let mut indent = 0;
        let file_name = "configuration.nix".to_string();
        let (tx, rx) = mpsc::channel();
        tui.trans = Some(rx);

        tx.send(ParseEvent::StartParsingFile(file_name.clone())).unwrap();
        tui.last_update = Instant::now() - Duration::from_millis(10);
        tui.parse_events(&mut indent);

        tx.send(ParseEvent::EndParsingFile(file_name)).unwrap();
        tui.last_update = Instant::now() - Duration::from_millis(10);
        tui.parse_events(&mut indent);

        assert_eq!(tui.logs.len(), 1);
        assert_eq!(tui.logs[0].status, TaskStatus::Finished);
        assert_eq!(tui.logs[0].name, "Generating AST: configuration.nix");
    }

    #[test]
    fn test_tui_core_load() {
        let mut tui = Tui::new();
        tui.load("/testestestestest");
        assert_eq!(tui.path, String::from("/testestestestest"));
        assert!(tui.trans.is_some());
        assert!(tui.time_rx.is_some());
    }

    fn assert_events(event1: ParseEvent, event2: Option<ParseEvent>, len1: usize, len2: usize, ind1: usize, ind2: usize) {
        let mut tui = Tui::new();
        let mut indent = 0;
        let (tx, rx) = mpsc::channel();
        tui.trans = Some(rx);
        tx.send(event1).unwrap();
        tui.last_update = Instant::now() - Duration::from_millis(10);
        tui.parse_events(&mut indent);

        assert_eq!(tui.logs.len(), len1);
        if len1 != 0 {
            assert_eq!(tui.logs[0].status, TaskStatus::Running);
        }
        assert_eq!(indent, ind1);

        if let Some(ev) = event2 {
            tx.send(ev).unwrap();
        }
        tui.last_update = Instant::now() - Duration::from_millis(10);
        tui.parse_events(&mut indent);

        assert_eq!(tui.logs.len(), len2);
        assert_eq!(indent, ind2);
    }

    #[test]
    fn test_tui_core_parse_events_attr_set() {
        assert_events(ParseEvent::StartAttrSet, Some(ParseEvent::EndAttrSet), 1, 0, 1, 0);
    }

    #[test]
    fn test_tui_core_parse_events_list() {
        assert_events(ParseEvent::StartList, Some(ParseEvent::EndList), 1, 0, 1, 0);
    }

    #[test]
    fn test_tui_core_parse_events_let_in() {
        assert_events(ParseEvent::StartLetIn, Some(ParseEvent::EndLetIn), 1, 0, 1, 0);
    }

    #[test]
    fn test_tui_core_parse_events_lambda() {
        assert_events(ParseEvent::StartLambda, Some(ParseEvent::EndLambda), 1, 0, 1, 0);
    }

    #[test]
    fn test_tui_core_parse_events_with() {
        assert_events(ParseEvent::StartWith, Some(ParseEvent::EndWith), 1, 0, 1, 0);
    }

    #[test]
    fn test_tui_core_parse_events_string() {
        assert_events(ParseEvent::StartString, Some(ParseEvent::EndString), 1, 0, 1, 0);
    }

    #[test]
    fn test_tui_core_parse_events_path() {
        assert_events(ParseEvent::StartPath, Some(ParseEvent::EndPath), 1, 0, 1, 0);
    }

    #[test]
    fn test_tui_core_parse_events_number() {
        assert_events(ParseEvent::StartNumber, Some(ParseEvent::EndNumber), 1, 0, 1, 0);
    }

    #[test]
    fn test_tui_core_parse_events_expression() {
        assert_events(ParseEvent::StartExpression, Some(ParseEvent::EndExpression), 1, 0, 1, 0);
    }

    #[test]
    fn test_tui_core_parse_events_operator() {
        assert_events(ParseEvent::StartOperator, Some(ParseEvent::EndOperator), 1, 0, 1, 0);
    }

    #[test]
    fn test_tui_core_parse_events_iddentifier() {
        assert_events(ParseEvent::StartIdentifier, Some(ParseEvent::EndIdentifier), 1, 0, 1, 0);
    }

    #[test]
    fn test_tui_core_parse_events_whitespace() {
        assert_events(ParseEvent::StartWhitespace, Some(ParseEvent::EndWhitespace), 1, 0, 1, 0);
    }
    
    #[test]
    fn test_tui_core_parse_events_value() {
        assert_events(ParseEvent::StartValue, Some(ParseEvent::EndValue), 1, 0, 1, 0);
    }

    #[test]
    fn test_tui_core_parse_events_group() {
        assert_events(ParseEvent::StartGroup, Some(ParseEvent::EndGroup), 1, 0, 1, 0);
    }

    #[test]
    fn test_tui_core_parse_events_antiquotation() {
        assert_events(ParseEvent::StartAntiquotation, Some(ParseEvent::EndAntiquotation), 1, 0, 1, 0);
    }

    #[test]
    fn test_tui_core_parse_events_indented_string() {
        assert_events(ParseEvent::StartIndentedString, Some(ParseEvent::EndIndentedString), 1, 0, 1, 0);
    }

    #[test]
    fn test_tui_core_parse_events_gen() {
        assert_events(ParseEvent::StartGen, Some(ParseEvent::EndGen), 1, 1, 1, 0);
    }

    #[test]
    fn test_tui_core_parse_events_getting_files() {
        assert_events(ParseEvent::StartGettingFiles, Some(ParseEvent::EndGettingFiles), 1, 0, 1, 0);
    }

    #[test]
    fn test_tui_core_parse_events_finished() {
        assert_events(ParseEvent::Finished("test".to_string()), None, 0, 0, 0, 0);
    }

    #[test]
    fn test_tui_core_parse_events_sorting_files() {
        assert_events(ParseEvent::StartSortingFile("test".to_string()), Some(ParseEvent::EndSortingFile("test".to_string())), 1, 0, 1, 0);
    }

    #[test]
    fn test_tui_core_parse_events_expanding_file() {
        assert_events(ParseEvent::StartExpandingFile("test".to_string()), Some(ParseEvent::EndExpandingFile("test".to_string())), 1, 0, 1, 0);
    }

    #[test]
    fn test_tui_core_parse_events_flattening_file() {
        assert_events(ParseEvent::StartFlatteningFile("test".to_string()), Some(ParseEvent::EndFlatteningFile("test".to_string())), 1, 0, 1, 0);
    }
}
