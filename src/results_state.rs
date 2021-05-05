use tui::widgets::ListState;

#[derive(Debug)]
pub struct ResultsState<'results> {
    pub list_state: ListState,
    pub items: &'results [String],
}

impl<'results> ResultsState<'results> {
    pub fn from_results(results: &'results [String]) -> Self {
        Self {
            list_state: ListState::default(),
            items: results,
        }
    }

    pub fn next(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.list_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.list_state.select(Some(i));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_allows_to_move_forwards_in_the_results_list() {
        let results = [
            String::from("result0"),
            String::from("result1"),
            String::from("result2"),
        ];
        let mut results_state = ResultsState::from_results(&results);

        assert_eq!(results_state.list_state.selected(), None);

        results_state.next();
        assert_eq!(results_state.list_state.selected(), Some(0));

        results_state.next();
        assert_eq!(results_state.list_state.selected(), Some(1));

        results_state.next();
        assert_eq!(results_state.list_state.selected(), Some(2));

        results_state.next();
        assert_eq!(results_state.list_state.selected(), Some(0));
    }

    #[test]
    fn it_allows_to_move_backwards_in_the_results_list() {
        let results = [
            String::from("result0"),
            String::from("result1"),
            String::from("result2"),
        ];
        let mut results_state = ResultsState::from_results(&results);

        assert_eq!(results_state.list_state.selected(), None);

        results_state.previous();
        assert_eq!(results_state.list_state.selected(), Some(0));

        results_state.previous();
        assert_eq!(results_state.list_state.selected(), Some(2));

        results_state.previous();
        assert_eq!(results_state.list_state.selected(), Some(1));

        results_state.previous();
        assert_eq!(results_state.list_state.selected(), Some(0));
    }
}
