use cursive::theme::Effect;
use cursive::utils::markup::StyledString;
use cursive::utils::span::SpannedString;
use cursive::views::{EditView, TextView};
use cursive::Cursive;

use crate::model;

pub fn refresh(model: &model::Model, siv: &mut Cursive, new_contents: &str) {
    siv.call_on_id("performance", |view: &mut TextView| {
        view.set_content(format!(".{}.", new_contents));
    });
    siv.call_on_id("display", |view: &mut TextView| {
        view.set_content(styled_string(&model, new_contents));
    });
    siv.call_on_id("entry", |view: &mut EditView| {
        view.set_content(new_contents);
    });
}

pub fn styled_string(model: &model::Model, entry: &str) -> StyledString {
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
    let mut styled_string = SpannedString::new();
    let words = model.get_words();
    for i in 0..words.len() {
        match i {
            0 => {
                let (prefix, suffix) = common_prefix(
                    &words.get(0).unwrap().to_string(),
                    &entry.to_string(),
                );
                styled_string
                    .append(SpannedString::styled(prefix, Effect::Simple));
                styled_string
                    .append(SpannedString::styled(suffix, Effect::Reverse));
            }
            _ => {
                styled_string.append(words.get(i).unwrap().to_string());
            }
        }
        styled_string.append(" ");
    }
    styled_string
}
