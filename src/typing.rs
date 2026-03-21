use crate::state::{CharResult, ResultsData, TypingState};
use std::time::Instant;

impl TypingState {
    pub fn handle_char(&mut self, ch: char) {
        if self.is_complete() {
            return;
        }

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

        // snapshot wpm and accuracy at each completed second
        if let Some(start) = self.started_at {
            let completed_secs = start.elapsed().as_secs_f32().floor() as usize;
            while self.wpm_history.len() < completed_secs {
                self.wpm_history.push(self.wpm());
                self.accuracy_history.push(self.accuracy());
            }
        }
    }

    pub fn handle_backspace(&mut self) {
        if self.cursor == 0 {
            return;
        }
        self.cursor -= 1;
        self.input.pop();
    }

    pub fn elapsed_secs(&self) -> f32 {
        match (self.started_at, self.finished_at) {
            (Some(start), Some(end)) => end.duration_since(start).as_secs_f32(),
            (Some(start), None) => start.elapsed().as_secs_f32(),
            _ => 0.0,
        }
    }

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
        let correct = self
            .input
            .iter()
            .filter(|&&r| r == CharResult::Correct)
            .count();
        (correct as f32 / self.input.len() as f32) * 100.0
    }

    pub fn error_count(&self) -> usize {
        self.input
            .iter()
            .filter(|&&r| r == CharResult::Incorrect)
            .count()
    }

    pub fn into_results_cloned(&self) -> ResultsData {
        let mut wpm_history = self.wpm_history.clone();
        let mut accuracy_history = self.accuracy_history.clone();
        wpm_history.push(self.wpm());
        accuracy_history.push(self.accuracy());
        ResultsData {
            wpm: self.wpm(),
            accuracy: self.accuracy(),
            time_elapsed: self.elapsed_secs(),
            language: self.language.clone(),
            snippet_length: self.snippet_length,
            errors: self.error_count(),
            wpm_history,
            accuracy_history,
        }
    }
}
