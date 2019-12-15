use std::collections::VecDeque;
use std::iter;
use std::iter::FusedIterator;

use super::consts;
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
                .take(consts::PANEL_ROWS)
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

    pub fn get_current_word(&self) -> Option<String> {
        let i = self.history.len();
        self.get_words().get(i).map(|word| word.to_owned())
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
            let current_width = acc.iter().map(String::len).sum::<usize>() + acc.len();
            if current_width + next.len() > consts::PANEL_WIDTH {
                Err(acc)
            } else {
                Ok(acc)
            }
        })
        .unwrap_or_else(|acc| acc)
    }
}

pub struct ModelBuilder<I>
where
    I: FusedIterator<Item = String>,
{
    word_stream: Option<I>,
}

impl<I> ModelBuilder<I>
where
    I: FusedIterator<Item = String>,
{
    pub fn new() -> ModelBuilder<I> {
        ModelBuilder { word_stream: None }
    }

    pub fn with_word_stream(mut self, word_stream: I) -> ModelBuilder<I> {
        self.word_stream = Some(word_stream);
        self
    }

    pub fn build(self) -> Model<I> {
        Model::new(self.word_stream.unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_values() {
        let lexicon = vec![
            "sphinx".to_string(),
            "of".to_string(),
            "black".to_string(),
            "quartz".to_string(),
        ];
        let stream = lexicon.iter().cloned().cycle();
        let model = ModelBuilder::new().with_word_stream(stream).build();
        assert_eq!(model.get_history().len(), 0);
        assert_eq!(model.get_current_word(), Some("sphinx".to_string()));
    }

    #[test]
    fn test_register() {
        let lexicon = vec![
            "sphinx".to_string(),
            "of".to_string(),
            "black".to_string(),
            "quartz".to_string(),
        ];
        let stream = lexicon.iter().cloned().cycle();
        let mut model = ModelBuilder::new().with_word_stream(stream).build();
        model.register("sphx");
        assert_eq!(model.get_history().len(), 1);
        assert_eq!(model.get_history().first(), Some(&"sphx".to_string()));
        assert_eq!(model.get_current_word(), Some("of".to_string()));
    }
}
