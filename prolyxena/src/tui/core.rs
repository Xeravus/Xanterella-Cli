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
    Terminal,
};
use std::{
    sync::mpsc, thread, time::Duration
};

pub enum Events {
    ParsingStarted(String),
    Log(String),
    Finished,
}

pub struct AppState {
    logs: Vec<String>,
}

pub fn start_tui() -> Result<(), Box<dyn std::error::Error>> {
    let mut state = AppState {
        logs: Vec::new()
    };
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        tx.send(Events::ParsingStarted("hardware-configuration.nix".to_string())).unwrap();
        thread::sleep(Duration::from_millis(800));

        tx.send(Events::Log("Baue CST...".to_string())).unwrap();
        thread::sleep(Duration::from_millis(900));

        tx.send(Events::Log("Extrahiere Hostname".to_string())).unwrap();
        thread::sleep(Duration::from_millis(500));

        tx.send(Events::Finished).unwrap()
    });
    enable_raw_mode()?; 
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?; 
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        while let Ok(event) = rx.try_recv() {
            match event {
                Events::ParsingStarted(file) => state.logs.push(format!("Lese Datei: {}", file)),
                Events::Log(log) => state.logs.push(format!("-> {}", log)),
                Events::Finished => state.logs.push(format!("Finished")),
            }
        }
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(10),
                    Constraint::Percentage(90),
                ])
                .split(f.size());
            let text = format!("Pfad: ~/xanterella/... \nTime: 0.052s");
            let log_text = state.logs.join("\n");
            let sidebar_block = Block::default()
                .title(" Prolyxena Parsegraph ")
                .borders(Borders::ALL);
            let main_block = Block::default()
                .title(" Prolyxena Output ")
                .borders(Borders::ALL);

            let sidebar_text = Paragraph::new(text)
                .block(sidebar_block);
            let main_text = Paragraph::new("Warte auf ParseEvents")
                .block(main_block);

            f.render_widget(Paragraph::new(log_text), chunks[1]);
            f.render_widget(sidebar_text, chunks[0]);
        })?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    break; 
                }
            }
        }
    }
    disable_raw_mode()?; 
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen 
    )?;
    terminal.show_cursor()?;
    Ok(())
}
