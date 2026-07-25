use crate::tui::core::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tui_core_new() {
        let mut data = Tui::new();

        assert!(data.logs.is_empty());
        assert!(data.path.is_empty());
        assert!(data.time.is_none());
        assert!(data.trans.is_none());
    }

    #[test]
    fn test_tui_core_parse_events() {
        let mut data1 = Tui::new();
        let mut data2 = Tui::new();
        let mut data3 = Tui::new();

        let mut indent1: usize = 0;
        let mut indent2: usize = 0;
        let mut indent3: usize = 0;

        let (tx1, rx1) = mpsc::channel::<ParseEvent>();
        let (tx2, rx2) = mpsc::channel::<ParseEvent>();
        let (tx3, rx3) = mpsc::channel::<ParseEvent>();

        data1.inject_trans(rx1);
        data2.inject_trans(rx2);
        data3.inject_trans(rx3);

        tx1.send(ParseEvent::StartAttrSet).ok();
        tx2.send(ParseEvent::StartAttrSet).ok();
        tx2.send(ParseEvent::EndAttrSet).ok();
        tx3.send(ParseEvent::StartAttrSet).ok();
        tx3.send(ParseEvent::StartValue).ok();
        tx3.send(ParseEvent::EndValue).ok();
        tx3.send(ParseEvent::EndAttrSet).ok();

        data1.parse_events(&mut indent1);
        data2.parse_events(&mut indent2);
        data3.parse_events(&mut indent3);

        let expected1 = vec![
            ParseTask {
                name: "Attribut Set".to_string(),
                indent: 0,
                status: TaskStatus::Running,
            }
        ];
        let expected2 = vec![
            ParseTask {
                name: "Attribut Set".to_string(),
                indent: 0,
                status: TaskStatus::Running,
            },
        ];
        let expected3 = vec![
            ParseTask {
                name: "Attribut Set".to_string(),
                indent: 0,
                status: TaskStatus::Running,
            },
            ParseTask {
                name: "Value".to_string(),
                indent: 1,
                status: TaskStatus::Running,
            },
        ];

        assert_eq!(data1.logs, expected1);
        assert_eq!(data2.logs, expected2);
        assert_eq!(data3.logs, expected3);
    }
}
