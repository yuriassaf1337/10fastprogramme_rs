use std::time::Instant;
use crate::state::{CharResult, TypingState, ResultsData};

impl TypingState {
    pub fn handle_char(&mut self, ch: char) {
        if self.is_complete() {
            return;
        }

        // start timer on first keystroke
        if self.started_at.is_none() {
            self.started_at = Some(Instant::now());
        }

        let expected = self.snippet[self.cursor];
        let result = if ch == expected {
            CharResult::Correct
        } else {
            CharResult::Incorrect
        };

        self.input.push(result);
        self.cursor += 1;

        if self.is_complete() {
            self.finished_at = Some(Instant::now());
        }
    }

    pub fn handle_backspace(&mut self) {
        if self.cursor == 0 {
            return;
        }
        self.cursor -= 1;
        self.input.pop();
    }

    /// returns elapsed seconds since typing began
    pub fn elapsed_secs(&self) -> f32 {
        match (self.started_at, self.finished_at) {
            (Some(start), Some(end)) => end.duration_since(start).as_secs_f32(),
            (Some(start), None)      => start.elapsed().as_secs_f32(),
            _                        => 0.0,
        }
    }

    /// gross WPM: (chars_typed / 5) / elapsed_minutes
    pub fn wpm(&self) -> f32 {
        let elapsed_mins = self.elapsed_secs() / 60.0;
        if elapsed_mins < 0.0001 {
            return 0.0;
        }
        (self.cursor as f32 / 5.0) / elapsed_mins
    }

    pub fn accuracy(&self) -> f32 {
        if self.input.is_empty() {
            return 100.0;
        }
        let correct = self.input.iter().filter(|&&r| r == CharResult::Correct).count();
        (correct as f32 / self.input.len() as f32) * 100.0
    }

    pub fn error_count(&self) -> usize {
        self.input.iter().filter(|&&r| r == CharResult::Incorrect).count()
    }

    pub fn into_results(self) -> ResultsData {
        ResultsData {
            wpm:          self.wpm(),
            accuracy:     self.accuracy(),
            time_elapsed: self.elapsed_secs(),
            language:     self.language,
            errors:       self.error_count(),
            chars_typed:  self.input.len(),
        }
    }

    /// sme as into_results but borrows self
    pub fn into_results_cloned(&self) -> ResultsData {
        ResultsData {
            wpm:          self.wpm(),
            accuracy:     self.accuracy(),
            time_elapsed: self.elapsed_secs(),
            language:     self.language,
            errors:       self.error_count(),
            chars_typed:  self.input.len(),
        }
    }
}
