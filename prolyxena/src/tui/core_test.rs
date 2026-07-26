use crate::tui::core::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    #[test]
    fn test_tui_core_new() {
        let mut data = Tui::new();

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
}
