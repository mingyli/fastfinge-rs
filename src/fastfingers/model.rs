use std::collections::VecDeque;
use std::iter;
use std::iter::FusedIterator;

use super::peeking::PeekingFoldWhileTrait;

type Row = Vec<String>;

#[derive(Debug)]
pub struct Model<I: FusedIterator<Item = String>> {
    words: VecDeque<Row>,
    history: Vec<String>,
    sampler: std::iter::Peekable<I>,
}

impl<I> Model<I>
where
    I: FusedIterator<Item = String>,
{
    pub fn new(it: I) -> Model<I> {
        let mut sampler = it.peekable();
        Model {
            words: iter::repeat_with(|| Model::make_row(&mut sampler))
                .take(crate::PANEL_ROWS)
                .collect::<VecDeque<Row>>(),
            history: Vec::new(),
            sampler,
        }
    }

    pub fn get_history(&self) -> &Vec<String> {
        &self.history
    }

    pub fn register(&mut self, entry: &str) {
        self.history.push(entry.to_owned());
        if self.history.len() == self.first_row().len() {
            self.advance();
            self.history.clear();
        }
    }

    fn advance(&mut self) {
        let row = self.get_row();
        self.words.push_back(row);
        self.words.pop_front();
    }

    pub fn get_words(&self) -> Vec<String> {
        self.words.iter().flatten().cloned().collect()
    }

    fn first_row(&self) -> &Row {
        self.words.front().unwrap()
    }

    fn get_row(&mut self) -> Row {
        Model::make_row(&mut self.sampler)
    }

    fn make_row(it: &mut std::iter::Peekable<I>) -> Row {
        it.peeking_fold_while(Vec::new(), |mut acc, (curr, peek)| {
            acc.push(curr.clone());
            let next = peek.expect("The provided sampler should not end.");
            let current_width =
                acc.iter().map(String::len).sum::<usize>() + acc.len();
            if current_width + next.len() > crate::PANEL_WIDTH {
                Err(acc)
            } else {
                Ok(acc)
            }
        })
        .unwrap_or_else(|acc| acc)
    }
}
