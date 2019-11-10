use cursive::traits::*;
use rand;
use rand::seq::SliceRandom;
use std::collections::VecDeque;


#[derive(Debug)]
pub struct Model<'a> {
    lexicon: Vec<&'a str>,
    words: VecDeque<&'a str>,
    history: Vec<&'a str>,
}

impl<'a> Model<'a> {
    pub fn new() -> Model<'a> {
        Model {
            lexicon: Vec::new(),
            words: VecDeque::new(),
            history: Vec::new(),
        }
    }

    pub fn with_lexicon(self, lexicon: Vec<&'a str>) -> Model<'a> {
        self.with(|s| s.set_lexicon(lexicon))
    }
    fn set_lexicon(&mut self, lexicon: Vec<&'a str>) {
        self.lexicon = lexicon.clone()
    }

    pub fn with_size(self, size: usize) -> Model<'a> {
        self.with(|s| s.set_size(size))
    }
    fn set_size(&mut self, size: usize) {
        let mut rng = rand::thread_rng();
        let slice = &self.lexicon[..];
        self.words = VecDeque::from(
            slice
                .choose_multiple(&mut rng, size)
                .cloned()
                .collect::<Vec<&'a str>>(),
        )
    }

    fn sample(&self) -> &'a str {
        let mut rng = rand::thread_rng();
        let slice = &self.lexicon[..];
        slice.choose(&mut rng).unwrap()
    }

    pub fn advance(&mut self) {
        self.words.push_back(self.sample());
        self.words.pop_front();
    }

    pub fn get_words(&self) -> VecDeque<&'a str> {
        self.words.clone()
    }

    fn display_string(&self) -> String {
        self.words
            .clone()
            .into_iter()
            .collect::<Vec<&str>>()
            .join(" ")
    }
}
