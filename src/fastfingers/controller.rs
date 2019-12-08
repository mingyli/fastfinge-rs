use cursive::Cursive;

use crate::fastfingers::model;
use crate::fastfingers::view;

pub fn on_edit(
    model: &mut model::Model,
    siv: &mut Cursive,
    contents: &str,
    _cursor: usize,
) {
    let mut new_contents = contents;
    if !contents.is_empty() {
        let input: char = contents.chars().last().unwrap();
        if input == ' ' {
            new_contents = "";
            model.advance(contents.trim());
        }
    }
    view::refresh(model, siv, &new_contents);
}
