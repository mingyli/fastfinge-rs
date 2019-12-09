use cursive::Cursive;

use crate::fastfingers::model;
use crate::fastfingers::view;

pub fn on_edit(
    model: &mut model::Model,
    siv: &mut Cursive,
    mut contents: &str,
    _cursor: usize,
) {
    if !contents.is_empty() {
        let keypress: char = contents.chars().last().unwrap();
        if keypress.is_whitespace() {
            model.register(contents.trim());
            contents = "";
        }
    }
    view::update(model, siv, &contents);
}
