use indicatif::{ProgressBar, ProgressStyle};

use std::thread::sleep;
use std::time::Duration;

use crate::engine::core::*;

pub fn show_parse_timeline(vec: Vec<ParseEvent>) {
    let pb = ProgressBar::new_spinner();

    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("XanterellaF")
            .template("{spinner:.green} [{elapsed_precise}] {msg}")
            .unwrap(),
        );

    for i in vec {
        let (is_start, name) = match i {
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

            ParseEvent::StartIdentifier => (true, "Identifier"),
            ParseEvent::EndIdentifier => (false, "Identifier"),

            ParseEvent::StartWhitespace => (true, "Whitespace"),
            ParseEvent::EndWhitespace => (false, "Whitespace"),

            ParseEvent::StartValue => (true, "Value"),
            ParseEvent::EndValue => (false, "Value"),
        };

        let action = if is_start { 
            "Öffne" 
        } else {
            "Schließe"
        };

        pb.set_message(format!("{} {}", action, name));
        pb.tick();
        // sleep(Duration::from_millis(2));
    }
    pb.finish_with_message(format!("Erfolgreich geparst"));
}
