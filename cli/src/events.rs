use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::results_state::ResultsState;
use anyhow::Result;
pub struct Events;

type ExitApp = bool;

impl Events {
    pub fn read(results_state: &mut ResultsState) -> Result<ExitApp> {
        if let Event::Key(event) = read()? {
            let exit_app = Self::handle_event(event, results_state);
            return Ok(exit_app);
        }
        Ok(false)
    }

    fn handle_event(key_event: KeyEvent, results_state: &mut ResultsState) -> ExitApp {
        match key_event {
            KeyEvent {
                code: KeyCode::Up, ..
            } => results_state.previous(),
            KeyEvent {
                code: KeyCode::Down,
                ..
            } => results_state.next(),
            KeyEvent {
                code: KeyCode::Esc, ..
            } => return true,
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => return true,
            _ => {}
        };
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_goes_forward_when_pressing_down_not_exiting_app() {
        let results = [String::from("a_string")];
        let mut results_state = ResultsState::from_results(&results);
        let key_event = KeyEvent::new(KeyCode::Down, KeyModifiers::NONE);

        let exit_app = Events::handle_event(key_event, &mut results_state);

        assert!(!exit_app);
        assert_eq!(results_state.list_state.selected(), Some(0));
    }

    #[test]
    fn it_goes_backward_when_pressing_down_not_exiting_app() {
        let results = [String::from("a_string")];
        let mut results_state = ResultsState::from_results(&results);
        let key_event = KeyEvent::new(KeyCode::Up, KeyModifiers::NONE);

        let exit_app = Events::handle_event(key_event, &mut results_state);

        assert!(!exit_app);
        assert_eq!(results_state.list_state.selected(), Some(0));
    }

    #[test]
    fn it_ignores_keys_with_no_action() {
        let results = [String::from("a_string")];
        let mut results_state = ResultsState::from_results(&results);
        let key_event = KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE);

        let exit_app = Events::handle_event(key_event, &mut results_state);

        assert!(!exit_app);
        assert_eq!(results_state.list_state.selected(), None);

        let key_event = KeyEvent::new(KeyCode::Char('A'), KeyModifiers::NONE);

        let exit_app = Events::handle_event(key_event, &mut results_state);

        assert!(!exit_app);
        assert_eq!(results_state.list_state.selected(), None);
    }

    #[test]
    fn it_exits_app_when_pressing_escape() {
        let results = [];
        let mut results_state = ResultsState::from_results(&results);
        let key_event = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);

        let exit_app = Events::handle_event(key_event, &mut results_state);

        assert!(exit_app);
    }

    #[test]
    fn it_exits_app_when_pressing_ctrl_c() {
        let results = [];
        let mut results_state = ResultsState::from_results(&results);
        let key_event = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);

        let exit_app = Events::handle_event(key_event, &mut results_state);

        assert!(exit_app);
    }
}
