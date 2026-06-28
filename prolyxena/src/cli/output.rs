use std::thread::sleep;
use std::time::Duration;

use crate::engine::core::*;

pub fn show_parse_timeline(vec: Vec<ParseEvent>) {
    for i in vec {
        match i {
            // Start
            ParseEvent::StartAttrSet => println!("Event: Starte Attribut Set"),
            ParseEvent::StartList => println!("Event: Starte Liste"),
            ParseEvent::StartLetIn => println!("Event: Starte Let-In"),
            ParseEvent::StartLambda => println!("Event: Starte Lambda"),
            ParseEvent::StartWith => println!("Event: Starte With"),
            ParseEvent::StartString => println!("Event: Starte String"),
            ParseEvent::StartPath => println!("Event: Starte Path"),
            ParseEvent::StartNumber => println!("Event: Starte Number"),
            ParseEvent::StartIdentifier => println!("Event: Starte Identifier"),
            ParseEvent::StartWhitespace => println!("Event: Starte Whitespace"),
            ParseEvent::StartValue => println!("Event: Starte Value"),
            // End 
            ParseEvent::EndAttrSet => println!("Event: Ende Attribut Set"),
            ParseEvent::EndList => println!("Event: End Liste"),
            ParseEvent::EndLetIn => println!("Event: End Let-In"),
            ParseEvent::EndLambda => println!("Event: End Lambda"),
            ParseEvent::EndWith => println!("Event: End With"),
            ParseEvent::EndString => println!("Event: End String"),
            ParseEvent::EndPath => println!("Event: End Path"),
            ParseEvent::EndNumber => println!("Event: End Number"),
            ParseEvent::EndIdentifier => println!("Event: End Identifier"),
            ParseEvent::EndWhitespace => println!("Event: End Whitespace"),
            ParseEvent::EndValue => println!("Event: End Whitespace"),
        }
        sleep(Duration::from_millis(10));
    }
}
