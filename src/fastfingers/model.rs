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

    pub fn with_lexicon(mut self, lexicon: &[String]) -> Model {
        self.words.set_lexicon(lexicon);
        self
    }

    pub fn with_width(mut self, width: usize) -> Model {
        self.words.set_width(width);
        self
    }

    pub fn width(&self) -> usize {
        self.words.width()
    }

    pub fn position(&self) -> usize {
        self.history.len()
    }

    pub fn get_history(&self) -> Vec<String> {
        self.history.clone()
    }

    pub fn register(&mut self, entry: &str) {
        self.history.push(entry.to_owned());
        if self.position() == self.words.width() {
            self.words.advance();
            self.history.clear();
        }
    }

    pub fn get_words(&self) -> Vec<String> {
        self.words.as_vec()
    }
}
