use cursive::theme::{BaseColor, Color, Effect, Style};
use cursive::utils::markup::StyledString;
use cursive::utils::span::SpannedString;
use cursive::views::{EditView, TextView};
use cursive::Cursive;
use std::cmp::Ordering;

use crate::fastfingers::model;

lazy_static! {
    static ref CORRECT_STYLE: Style = Style::from(Effect::Bold);
    static ref INCORRECT_STYLE: Style =
        Style::from(Effect::Reverse).combine(Color::Dark(BaseColor::Red));
    static ref CURRENT_STYLE: Style = Style::from(Effect::Reverse);
    static ref FUTURE_STYLE: Style = Style::from(Effect::Simple);
}

pub fn update(model: &model::Model, siv: &mut Cursive, current_word: &str) {
    siv.call_on_id("performance", |view: &mut TextView| {
        view.set_content(format!(".{}.", current_word));
    });
    siv.call_on_id("display", |view: &mut TextView| {
        view.set_content(get_styled_display(&model, current_word));
    });
    siv.call_on_id("entry", |view: &mut EditView| {
        view.set_content(current_word);
    });
}

fn common_prefix(s1: &str, s2: &str) -> (String, String) {
    let len = s1
        .chars()
        .zip(s2.chars())
        .take_while(|(x, y)| x == y)
        .count();
    let (prefix, suffix) = s1.split_at(len);
    (prefix.to_owned(), suffix.to_owned())
}

fn get_styled_words(
    words: &[String],
    history: &[String],
    current_word: &str,
) -> Vec<StyledString> {
    let get_entry = |pos: usize| match pos.cmp(&history.len()) {
        Ordering::Less => &history[pos],
        Ordering::Equal => current_word,
        Ordering::Greater => &words[pos],
    };
    let get_prefix_style = |pos: usize| match pos.cmp(&history.len()) {
        Ordering::Less => {
            if history[pos] != words[pos]
                && history[pos].starts_with(&words[pos])
            {
                *INCORRECT_STYLE
            } else {
                *CORRECT_STYLE
            }
        }
        Ordering::Equal => *CORRECT_STYLE,
        Ordering::Greater => *FUTURE_STYLE,
    };
    let get_suffix_style = |pos: usize| match pos.cmp(&history.len()) {
        Ordering::Less => *INCORRECT_STYLE,
        Ordering::Equal => *CURRENT_STYLE,
        Ordering::Greater => *FUTURE_STYLE,
    };
    (0..words.len())
        .map(|pos| {
            let entry: &str = get_entry(pos);
            let prefix_style: Style = get_prefix_style(pos);
            let suffix_style: Style = get_suffix_style(pos);
            let (prefix, suffix) = common_prefix(&words[pos], entry);
            (
                SpannedString::styled(prefix, prefix_style),
                SpannedString::styled(suffix, suffix_style),
            )
        })
        .map(|(styled_prefix, styled_suffix)| {
            let mut styled_word = SpannedString::new();
            styled_word.append(styled_prefix.clone());
            styled_word.append(styled_suffix.clone());
            styled_word
        })
        .collect()
}

fn get_styled_string(
    words: &[String],
    history: &[String],
    current_word: &str,
    row_width: usize,
) -> StyledString {
    let styled_words = get_styled_words(words, history, current_word);
    styled_words
        .chunks(row_width)
        .map(|chunk| {
            chunk
                .iter()
                .fold(SpannedString::new(), |mut acc, styled_word| {
                    acc.append(styled_word.clone());
                    acc.append(" ");
                    acc
                })
        })
        .fold(SpannedString::new(), |mut acc, row| {
            acc.append(row);
            acc.append("\n");
            acc
        })
}

pub fn get_styled_display(
    model: &model::Model,
    current_word: &str,
) -> StyledString {
    let words = model.get_words();
    let history = model.get_history();
    get_styled_string(&words, &history, current_word, model.width())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_prefix() {
        let (prefix, suffix) = common_prefix("asdfgjk", "asfjkli");
        assert_eq!(prefix, "as");
        assert_eq!(suffix, "dfgjk");
    }

    #[test]
    fn test_get_styled_string() {
        let words = ["sphinx", "of", "black", "quartz", "judge"]
            .iter()
            .cloned()
            .map(String::from)
            .collect::<Vec<String>>();
        let history = ["sphinx", "off", "blk"]
            .iter()
            .cloned()
            .map(String::from)
            .collect::<Vec<String>>();
        let styled_string: StyledString =
            get_styled_string(&words, &history, "qu", 3);
        let mut expected = SpannedString::new();
        expected.append(SpannedString::styled("sphinx", *CORRECT_STYLE));
        expected.append(SpannedString::styled("", *INCORRECT_STYLE));
        expected.append(" ");
        expected.append(SpannedString::styled("of", *INCORRECT_STYLE));
        expected.append(SpannedString::styled("", *INCORRECT_STYLE));
        expected.append(" ");
        expected.append(SpannedString::styled("bl", *CORRECT_STYLE));
        expected.append(SpannedString::styled("ack", *INCORRECT_STYLE));
        expected.append(" ");
        expected.append("\n");
        expected.append(SpannedString::styled("qu", *CORRECT_STYLE));
        expected.append(SpannedString::styled("artz", *CURRENT_STYLE));
        expected.append(" ");
        expected.append(SpannedString::styled("judge", *FUTURE_STYLE));
        expected.append(SpannedString::styled("", *FUTURE_STYLE));
        expected.append(" ");
        expected.append("\n");
        assert_eq!(styled_string, expected);
    }
}
