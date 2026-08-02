use crate::tui::core::*;

#[cfg(test)]
mod tests {
    use std::sync::mpsc;

    use super::*;
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

    #[test]
    fn test_tui_core_load() {
        let mut tui = Tui::new();
        tui.load("/testestestestest");
        assert_eq!(tui.path, String::from("/testestestestest"));
        assert!(tui.trans.is_some());
        assert!(tui.time_rx.is_some());
    }

    #[test]
    fn test_tui_core_parse_events_events() {
        let mut tui1 = Tui::new();
        let mut tui2 = Tui::new();
        let mut tui3 = Tui::new();
        let mut tui4 = Tui::new();
        let mut tui5 = Tui::new();
        let mut tui6 = Tui::new();
        let mut tui7 = Tui::new();
        let mut tui8 = Tui::new();
        let mut tui9 = Tui::new();
        let mut tui10 = Tui::new();
        let mut tui11 = Tui::new();
        let mut tui12 = Tui::new();
        let mut tui13 = Tui::new();
        let mut tui14 = Tui::new();
        let mut tui15 = Tui::new();
        let mut tui16 = Tui::new();
        let mut tui17 = Tui::new();
        let mut tui18 = Tui::new();

        let mut indent1 = 0;
        let mut indent2 = 0;
        let mut indent3 = 0;
        let mut indent4 = 0;
        let mut indent5 = 0;
        let mut indent6 = 0;
        let mut indent7 = 0;
        let mut indent8 = 0;
        let mut indent9 = 0;
        let mut indent10 = 0;
        let mut indent11 = 0;
        let mut indent12 = 0;
        let mut indent13 = 0;
        let mut indent14 = 0;
        let mut indent15 = 0;
        let mut indent16 = 0;
        let mut indent17 = 0;
        let mut indent18 = 0;

        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel();
        let (tx3, rx3) = mpsc::channel();
        let (tx4, rx4) = mpsc::channel();
        let (tx5, rx5) = mpsc::channel();
        let (tx6, rx6) = mpsc::channel();
        let (tx7, rx7) = mpsc::channel();
        let (tx8, rx8) = mpsc::channel();
        let (tx9, rx9) = mpsc::channel();
        let (tx10, rx10) = mpsc::channel();
        let (tx11, rx11) = mpsc::channel();
        let (tx12, rx12) = mpsc::channel();
        let (tx13, rx13) = mpsc::channel();
        let (tx14, rx14) = mpsc::channel();
        let (tx15, rx15) = mpsc::channel();
        let (tx16, rx16) = mpsc::channel();
        let (tx17, rx17) = mpsc::channel();
        let (tx18, rx18) = mpsc::channel();

        tui1.trans = Some(rx1);
        tui2.trans = Some(rx2);
        tui3.trans = Some(rx3);
        tui4.trans = Some(rx4);
        tui5.trans = Some(rx5);
        tui6.trans = Some(rx6);
        tui7.trans = Some(rx7);
        tui8.trans = Some(rx8);
        tui9.trans = Some(rx9);
        tui10.trans = Some(rx10);
        tui11.trans = Some(rx11);
        tui12.trans = Some(rx12);
        tui13.trans = Some(rx13);
        tui14.trans = Some(rx14);
        tui15.trans = Some(rx15);
        tui16.trans = Some(rx16);
        tui17.trans = Some(rx17);
        tui18.trans = Some(rx18);

        tx1.send(ParseEvent::StartAttrSet).unwrap();
        tx2.send(ParseEvent::StartList).unwrap();
        tx3.send(ParseEvent::StartLetIn).unwrap();
        tx4.send(ParseEvent::StartLambda).unwrap();
        tx5.send(ParseEvent::StartWith).unwrap();
        tx6.send(ParseEvent::StartString).unwrap();
        tx7.send(ParseEvent::StartPath).unwrap();
        tx8.send(ParseEvent::StartNumber).unwrap();
        tx9.send(ParseEvent::StartExpression).unwrap();
        tx10.send(ParseEvent::StartOperator).unwrap();
        tx11.send(ParseEvent::StartIdentifier).unwrap();
        tx12.send(ParseEvent::StartWhitespace).unwrap();
        tx13.send(ParseEvent::StartValue).unwrap();
        tx14.send(ParseEvent::StartGroup).unwrap();
        tx15.send(ParseEvent::StartAntiquotation).unwrap();
        tx16.send(ParseEvent::StartIndentedString).unwrap();
        tx17.send(ParseEvent::StartGen).unwrap();
        tx18.send(ParseEvent::StartGettingFiles).unwrap();

        tui1.last_update = Instant::now() - Duration::from_millis(10);
        tui2.last_update = Instant::now() - Duration::from_millis(10);
        tui3.last_update = Instant::now() - Duration::from_millis(10);
        tui4.last_update = Instant::now() - Duration::from_millis(10);
        tui5.last_update = Instant::now() - Duration::from_millis(10);
        tui6.last_update = Instant::now() - Duration::from_millis(10);
        tui7.last_update = Instant::now() - Duration::from_millis(10);
        tui8.last_update = Instant::now() - Duration::from_millis(10);
        tui9.last_update = Instant::now() - Duration::from_millis(10);
        tui10.last_update = Instant::now() - Duration::from_millis(10);
        tui11.last_update = Instant::now() - Duration::from_millis(10);
        tui12.last_update = Instant::now() - Duration::from_millis(10);
        tui13.last_update = Instant::now() - Duration::from_millis(10);
        tui14.last_update = Instant::now() - Duration::from_millis(10);
        tui15.last_update = Instant::now() - Duration::from_millis(10);
        tui16.last_update = Instant::now() - Duration::from_millis(10);
        tui17.last_update = Instant::now() - Duration::from_millis(10);
        tui18.last_update = Instant::now() - Duration::from_millis(10);

        tui1.parse_events(&mut indent1);
        tui2.parse_events(&mut indent2);
        tui3.parse_events(&mut indent3);
        tui4.parse_events(&mut indent4);
        tui5.parse_events(&mut indent5);
        tui6.parse_events(&mut indent6);
        tui7.parse_events(&mut indent7);
        tui8.parse_events(&mut indent8);
        tui9.parse_events(&mut indent9);
        tui10.parse_events(&mut indent10);
        tui11.parse_events(&mut indent11);
        tui12.parse_events(&mut indent12);
        tui13.parse_events(&mut indent13);
        tui14.parse_events(&mut indent14);
        tui15.parse_events(&mut indent15);
        tui16.parse_events(&mut indent16);
        tui17.parse_events(&mut indent17);
        tui18.parse_events(&mut indent18);

        assert_eq!(tui1.logs.len(), 1);
        assert_eq!(tui2.logs.len(), 1);
        assert_eq!(tui3.logs.len(), 1);
        assert_eq!(tui4.logs.len(), 1);
        assert_eq!(tui5.logs.len(), 1);
        assert_eq!(tui6.logs.len(), 1);
        assert_eq!(tui7.logs.len(), 1);
        assert_eq!(tui8.logs.len(), 1);
        assert_eq!(tui9.logs.len(), 1);
        assert_eq!(tui10.logs.len(), 1);
        assert_eq!(tui11.logs.len(), 1);
        assert_eq!(tui12.logs.len(), 1);
        assert_eq!(tui13.logs.len(), 1);
        assert_eq!(tui14.logs.len(), 1);
        assert_eq!(tui15.logs.len(), 1);
        assert_eq!(tui16.logs.len(), 1);
        assert_eq!(tui17.logs.len(), 1);
        assert_eq!(tui18.logs.len(), 1);

        assert_eq!(tui1.logs[0].status, TaskStatus::Running);
        assert_eq!(tui1.logs[0].status, TaskStatus::Running);
        assert_eq!(tui2.logs[0].status, TaskStatus::Running);
        assert_eq!(tui3.logs[0].status, TaskStatus::Running);
        assert_eq!(tui4.logs[0].status, TaskStatus::Running);
        assert_eq!(tui5.logs[0].status, TaskStatus::Running);
        assert_eq!(tui6.logs[0].status, TaskStatus::Running);
        assert_eq!(tui7.logs[0].status, TaskStatus::Running);
        assert_eq!(tui8.logs[0].status, TaskStatus::Running);
        assert_eq!(tui9.logs[0].status, TaskStatus::Running);
        assert_eq!(tui10.logs[0].status, TaskStatus::Running);
        assert_eq!(tui11.logs[0].status, TaskStatus::Running);
        assert_eq!(tui12.logs[0].status, TaskStatus::Running);
        assert_eq!(tui13.logs[0].status, TaskStatus::Running);
        assert_eq!(tui14.logs[0].status, TaskStatus::Running);
        assert_eq!(tui15.logs[0].status, TaskStatus::Running);
        assert_eq!(tui16.logs[0].status, TaskStatus::Running);
        assert_eq!(tui17.logs[0].status, TaskStatus::Running);
        assert_eq!(tui18.logs[0].status, TaskStatus::Running);

        assert_eq!(indent1, 1);
        assert_eq!(indent2, 1);
        assert_eq!(indent3, 1);
        assert_eq!(indent4, 1);
        assert_eq!(indent5, 1);
        assert_eq!(indent6, 1);
        assert_eq!(indent7, 1);
        assert_eq!(indent8, 1);
        assert_eq!(indent9, 1);
        assert_eq!(indent10, 1);
        assert_eq!(indent11, 1);
        assert_eq!(indent12, 1);
        assert_eq!(indent13, 1);
        assert_eq!(indent14, 1);
        assert_eq!(indent15, 1);
        assert_eq!(indent16, 1);
        assert_eq!(indent17, 1);
        assert_eq!(indent18, 1);

        tx1.send(ParseEvent::EndAttrSet).unwrap();
        tx2.send(ParseEvent::EndList).unwrap();
        tx3.send(ParseEvent::EndLetIn).unwrap();
        tx4.send(ParseEvent::EndLambda).unwrap();
        tx5.send(ParseEvent::EndWith).unwrap();
        tx6.send(ParseEvent::EndString).unwrap();
        tx7.send(ParseEvent::EndPath).unwrap();
        tx8.send(ParseEvent::EndNumber).unwrap();
        tx9.send(ParseEvent::EndExpression).unwrap();
        tx10.send(ParseEvent::EndOperator).unwrap();
        tx11.send(ParseEvent::EndIdentifier).unwrap();
        tx12.send(ParseEvent::EndWhitespace).unwrap();
        tx13.send(ParseEvent::EndValue).unwrap();
        tx14.send(ParseEvent::EndGroup).unwrap();
        tx15.send(ParseEvent::EndAntiquotation).unwrap();
        tx16.send(ParseEvent::EndIndentedString).unwrap();
        tx17.send(ParseEvent::EndGen).unwrap();
        tx18.send(ParseEvent::EndGettingFiles).unwrap();

        tui1.last_update = Instant::now() - Duration::from_millis(10);
        tui2.last_update = Instant::now() - Duration::from_millis(10);
        tui3.last_update = Instant::now() - Duration::from_millis(10);
        tui4.last_update = Instant::now() - Duration::from_millis(10);
        tui5.last_update = Instant::now() - Duration::from_millis(10);
        tui6.last_update = Instant::now() - Duration::from_millis(10);
        tui7.last_update = Instant::now() - Duration::from_millis(10);
        tui8.last_update = Instant::now() - Duration::from_millis(10);
        tui9.last_update = Instant::now() - Duration::from_millis(10);
        tui10.last_update = Instant::now() - Duration::from_millis(10);
        tui11.last_update = Instant::now() - Duration::from_millis(10);
        tui12.last_update = Instant::now() - Duration::from_millis(10);
        tui13.last_update = Instant::now() - Duration::from_millis(10);
        tui14.last_update = Instant::now() - Duration::from_millis(10);
        tui15.last_update = Instant::now() - Duration::from_millis(10);
        tui16.last_update = Instant::now() - Duration::from_millis(10);
        tui17.last_update = Instant::now() - Duration::from_millis(10);
        tui18.last_update = Instant::now() - Duration::from_millis(10);

        tui1.parse_events(&mut indent1);
        tui2.parse_events(&mut indent2);
        tui3.parse_events(&mut indent3);
        tui4.parse_events(&mut indent4);
        tui5.parse_events(&mut indent5);
        tui6.parse_events(&mut indent6);
        tui7.parse_events(&mut indent7);
        tui8.parse_events(&mut indent8);
        tui9.parse_events(&mut indent9);
        tui10.parse_events(&mut indent10);
        tui11.parse_events(&mut indent11);
        tui12.parse_events(&mut indent12);
        tui13.parse_events(&mut indent13);
        tui14.parse_events(&mut indent14);
        tui15.parse_events(&mut indent15);
        tui16.parse_events(&mut indent16);
        tui17.parse_events(&mut indent17);
        tui18.parse_events(&mut indent18);

        assert_eq!(tui1.logs.len(), 0);
        assert_eq!(tui2.logs.len(), 0);
        assert_eq!(tui3.logs.len(), 0);
        assert_eq!(tui4.logs.len(), 0);
        assert_eq!(tui5.logs.len(), 0);
        assert_eq!(tui6.logs.len(), 0);
        assert_eq!(tui7.logs.len(), 0);
        assert_eq!(tui8.logs.len(), 0);
        assert_eq!(tui9.logs.len(), 0);
        assert_eq!(tui10.logs.len(), 0);
        assert_eq!(tui11.logs.len(), 0);
        assert_eq!(tui12.logs.len(), 0);
        assert_eq!(tui13.logs.len(), 0);
        assert_eq!(tui14.logs.len(), 0);
        assert_eq!(tui15.logs.len(), 0);
        assert_eq!(tui16.logs.len(), 0);
        assert_eq!(tui17.logs.len(), 0);
        assert_eq!(tui18.logs.len(), 0);

        assert_eq!(indent1, 0);
        assert_eq!(indent2, 0);
        assert_eq!(indent3, 0);
        assert_eq!(indent4, 0);
        assert_eq!(indent5, 0);
        assert_eq!(indent6, 0);
        assert_eq!(indent7, 0);
        assert_eq!(indent8, 0);
        assert_eq!(indent9, 0);
        assert_eq!(indent10, 0);
        assert_eq!(indent11, 0);
        assert_eq!(indent12, 0);
        assert_eq!(indent13, 0);
        assert_eq!(indent14, 0);
        assert_eq!(indent15, 0);
        assert_eq!(indent16, 0);
        assert_eq!(indent17, 0);
        assert_eq!(indent18, 0);
    }
}
