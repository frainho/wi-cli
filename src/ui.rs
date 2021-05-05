#![allow(clippy::upper_case_acronyms)]
use std::io::{self, Stdout};

use anyhow::Result;
use crossterm::terminal::enable_raw_mode;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::Layout,
    style::{Color, Modifier, Style},
    terminal::CompletedFrame,
    text::{Span, Text},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};
use tui::{
    layout::{Constraint, Direction},
    Terminal,
};

use crate::ResultsState;

#[derive(Debug)]
pub struct UI<B: Backend> {
    terminal: Terminal<B>,
}

impl<B> UI<B>
where
    B: Backend,
{
    pub fn new(backend: B) -> Self {
        let mut terminal = match Terminal::new(backend) {
            Ok(terminal) => terminal,
            Err(error) => panic!("Unable to build terminal ui, {}", error),
        };
        if let Err(error) = terminal.clear() {
            panic!("Unable to clear the terminal, {}", error)
        };
        Self { terminal }
    }

    pub fn draw(&mut self, results: &mut ResultsState) -> Result<CompletedFrame> {
        let frame = self.terminal.draw(|f| {
            let layout_chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(f.size());

            let list_items = results
                .items
                .iter()
                .enumerate()
                .map(|(index, _)| {
                    let text = Span::styled(index.to_string(), Style::default().fg(Color::White));
                    ListItem::new(text)
                })
                .collect::<Vec<ListItem>>();

            let list = List::new(list_items)
                .block(Block::default().title("Files").borders(Borders::ALL))
                .highlight_style(
                    Style::default()
                        .bg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            f.render_stateful_widget(list, layout_chunks[1], &mut results.list_state);

            let file_content_block = Block::default().title("File Content").borders(Borders::ALL);
            let selected_file = results
                .items
                .get(results.list_state.selected().unwrap_or(0))
                .unwrap();
            let file_contents = Paragraph::new(Text::raw(selected_file)).block(file_content_block);
            f.render_widget(file_contents, layout_chunks[0]);
        })?;

        Ok(frame)
    }
}

impl Default for UI<CrosstermBackend<Stdout>> {
    fn default() -> Self {
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        if let Err(error) = enable_raw_mode() {
            panic!("Unable to enable raw mode: {}", error)
        };
        UI::new(backend)
    }
}

#[cfg(test)]
mod tests {
    use tui::{backend::TestBackend, buffer::Buffer};

    use super::*;

    #[test]
    #[ignore]
    fn it_draws() -> Result<()> {
        let test_backend = TestBackend::new(10, 10);
        let mut ui = UI::new(test_backend);
        let results = [
            String::from("test0"),
            String::from("test1"),
            String::from("test2"),
        ];
        let mut results_state = ResultsState::from_results(&results);

        let frame = ui.draw(&mut results_state)?;
        dbg!(&frame.buffer);
        let expected = Buffer::with_lines(vec![
            "          ",
            "┌File C┐  ",
            "│test0 │  ",
            "│      │  ",
            "│      │  ",
            "└──────┘  ",
            "┌Files─┐  ",
            "│0     │  ",
            "└──────┘  ",
            "          ",
        ]);
        dbg!(&expected);
        assert_eq!(expected.diff(&frame.buffer), vec![]);
        Ok(())
    }
}
