use rand;
use rand::seq::SliceRandom;
use std::collections::VecDeque;

const NUM_ROWS: usize = 2;

#[derive(Debug, Clone)]
pub struct WordQueue {
    width: usize,
    lexicon: Vec<String>,
    queue: VecDeque<String>,
}

impl WordQueue {
    pub fn new() -> WordQueue {
        WordQueue {
            width: 0,
            lexicon: Vec::new(),
            queue: VecDeque::new(),
        }
    }

    pub fn set_lexicon(&mut self, lexicon: &[String]) {
        self.lexicon = lexicon.to_vec()
    }

    pub fn set_width(&mut self, width: usize) {
        self.width = width;
        self.populate()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    fn populate(&mut self) {
        let mut rng = rand::thread_rng();
        let slice = &self.lexicon[..];
        self.queue = VecDeque::from(
            slice
                .choose_multiple(&mut rng, self.width * NUM_ROWS)
                .cloned()
                .collect::<Vec<String>>(),
        )
    }

    pub fn advance(&mut self) {
        let mut rng = rand::thread_rng();
        let slice = &self.lexicon[..];
        for _ in 0..self.width() {
            let sample = slice.choose(&mut rng).unwrap();
            self.queue.push_back(sample.to_owned());
            self.queue.pop_front();
        }
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn get(&self, i: usize) -> &String {
        self.queue.get(i).unwrap()
    }
}
