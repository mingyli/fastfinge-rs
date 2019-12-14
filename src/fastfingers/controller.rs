use std::iter::FusedIterator;

use cursive::Cursive;

use super::view;
use super::Model;
use super::PerformanceMonitor;

pub fn on_keypress<I>(
    model: &mut Model<I>,
    performance_monitor: &mut PerformanceMonitor,
    siv: &mut Cursive,
    mut contents: &str,
    _cursor: usize,
) where
    I: FusedIterator<Item = String>,
{
    if !contents.is_empty() {
        let keypress: char = contents.chars().last().unwrap();
        if keypress.is_whitespace() {
            contents = contents.trim();
            let expected = &model
                .get_current_word()
                .expect("There should be a current word.");
            model.register(contents);
            performance_monitor.register(contents, expected);
            contents = "";
        }
    }
    view::update(model, performance_monitor, siv, &contents);
}
