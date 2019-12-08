use cursive::traits::*;

use crate::fastfingers::word_queue;

#[derive(Debug)]
pub struct Model {
    words: word_queue::WordQueue,
    history: Vec<String>,
}

impl Model {
    pub fn new() -> Model {
        Model {
            words: word_queue::WordQueue::new(),
            history: Vec::new(),
        }
    }

    pub fn with_lexicon(self, lexicon: &[String]) -> Model {
        self.with(|s| s.set_lexicon(lexicon))
    }
    fn set_lexicon(&mut self, lexicon: &[String]) {
        self.words.set_lexicon(lexicon)
    }

    pub fn with_width(self, width: usize) -> Model {
        self.with(|s| s.set_width(width))
    }
    fn set_width(&mut self, width: usize) {
        self.words.set_width(width)
    }
    pub fn width(&self) -> usize {
        self.words.width()
    }

    pub fn position(&self) -> usize {
        self.history.len()
    }

    pub fn get_history(&self, i: usize) -> &String {
        self.history.get(i).unwrap()
    }

    pub fn advance(&mut self, entry: &str) {
        self.history.push(entry.to_owned());
        if self.position() == self.words.width() {
            self.words.advance();
            self.history.clear();
        }
    }

    pub fn get_words(&self) -> word_queue::WordQueue {
        self.words.clone()
    }
}
