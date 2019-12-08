use cursive::theme::Effect;
use cursive::utils::markup::StyledString;
use cursive::utils::span::SpannedString;
use cursive::views::{EditView, TextView};
use cursive::Cursive;

use crate::fastfingers::model;

pub fn refresh(model: &model::Model, siv: &mut Cursive, new_contents: &str) {
    siv.call_on_id("performance", |view: &mut TextView| {
        view.set_content(format!(".{}.", new_contents));
    });
    siv.call_on_id("display", |view: &mut TextView| {
        view.set_content(render_display(&model, new_contents));
    });
    siv.call_on_id("entry", |view: &mut EditView| {
        view.set_content(new_contents);
    });
}

pub fn render_display(model: &model::Model, content: &str) -> StyledString {
    fn common_prefix(s1: &str, s2: &str) -> (String, String) {
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
    let mut styled_string = SpannedString::new();
    let words = model.get_words();
    for i in 0..words.len() {
        let prefix_color: Effect = match i {
            i if i < model.position() => Effect::Italic,
            i if i == model.position() => Effect::Bold,
            _ => Effect::Simple,
        };
        let suffix_color: Effect = match i {
            i if i < model.position() => Effect::Underline,
            i if i == model.position() => Effect::Reverse,
            _ => Effect::Simple,
        };
        let entry: &str = match i {
            i if i < model.position() => &model.get_history(i),
            i if i == model.position() => content,
            _ => &words.get(i),
        };
        let (prefix, suffix) = common_prefix(&words.get(i), entry);
        styled_string.append(SpannedString::styled(prefix, prefix_color));
        styled_string.append(SpannedString::styled(suffix, suffix_color));

        if i == model.width() {
            styled_string.append("\n");
        } else {
            styled_string.append(" ");
        }
    }
    styled_string
}
