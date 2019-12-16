use cursive::Cursive;

use crate::model::Model;
use crate::performance::PerformanceMonitor;
use crate::view;

pub fn on_keypress<I>(
    siv: &mut Cursive,
    model: &mut Model<I>,
    performance_monitor: &mut PerformanceMonitor,
    mut contents: &str,
    _cursor: usize,
) where
    I: Iterator<Item = String>,
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
    view::update_model_display(siv, model, &contents);
    view::update_performance_display(siv, performance_monitor);
}

pub fn on_start<I>(siv: &mut Cursive, model: &Model<I>)
where
    I: Iterator<Item = String>,
{
    view::update_model_display(siv, model, "");
    view::update_display_on_start(siv);
}
