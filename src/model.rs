use cursive::theme::Effect;
use cursive::traits::*;
use cursive::utils::markup::StyledString;
use cursive::utils::span::SpannedString;
use cursive::views::{EditView, TextView};
use cursive::Cursive;
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

    fn advance(&mut self) {
        self.words.push_back(self.sample());
        self.words.pop_front();
    }

    pub fn on_edit(&mut self, s: &mut Cursive, contents: &str, _cursor: usize) {
        let mut new_contents = contents;
        if !contents.is_empty() {
            let input: char = contents.chars().last().unwrap();
            match input {
                ' ' => {
                    new_contents = "";
                    self.advance();
                }
                _ => (),
            }
        }
        s.call_on_id("performance", |view: &mut TextView| {
            view.set_content(format!(".{}.", new_contents));
        });
        s.call_on_id("display", |view: &mut TextView| {
            view.set_content(self.styled_string(new_contents));
        });
        s.call_on_id("entry", |view: &mut EditView| {
            view.set_content(new_contents);
        });
    }

    fn display_string(&self) -> String {
        self.words
            .clone()
            .into_iter()
            .collect::<Vec<&str>>()
            .join(" ")
    }

    pub fn styled_string(&self, entry: &str) -> StyledString {
        let mut styled_string = SpannedString::new();
        for i in 0..self.words.len() {
            match i {
                0 => {
                    let (prefix, suffix) = common_prefix(
                        &self.words.get(0).unwrap().to_string(),
                        &entry.to_string(),
                    );
                    styled_string
                        .append(SpannedString::styled(prefix, Effect::Simple));
                    styled_string
                        .append(SpannedString::styled(suffix, Effect::Reverse));
                }
                _ => {
                    styled_string
                        .append(self.words.get(i).unwrap().to_string());
                }
            }
            styled_string.append(" ");
        }
        styled_string
    }
}

fn common_prefix(s1: &String, s2: &String) -> (String, String) {
    let len = s1
        .chars()
        .zip(s2.chars())
        .take_while(|(x, y)| x == y)
        .count();
    (
        s1[..len].chars().collect::<String>(),
        s1[len..].chars().collect::<String>(),
    )
}
