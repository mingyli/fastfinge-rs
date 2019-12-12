use cursive::Cursive;
use std::iter::FusedIterator;

use crate::fastfingers::model::Model;
use crate::fastfingers::view;

pub fn on_edit<I>(
    model: &mut Model<I>,
    siv: &mut Cursive,
    mut contents: &str,
    _cursor: usize,
) where
    I: FusedIterator<Item = String>,
{
    if !contents.is_empty() {
        let keypress: char = contents.chars().last().unwrap();
        if keypress.is_whitespace() {
            model.register(contents.trim());
            contents = "";
        }
    }
    view::update(model, siv, &contents);
}
