use itertools::{Itertools, Position};
use std::cmp::Ordering;

use cursive::theme::Style;
use cursive::utils::markup::StyledString;
use cursive::utils::span::SpannedString;
use cursive::views::{EditView, HideableView, LinearLayout, StackView, TextView};
use cursive::Cursive;

use crate::consts;
use crate::model::Model;
use crate::performance::PerformanceMonitor;

pub fn update_model_display<I>(siv: &mut Cursive, model: &Model<I>, current_word: &str)
where
    I: Iterator<Item = String>,
{
    siv.call_on_id(consts::DISPLAY, |view: &mut TextView| {
        view.set_content(get_styled_display(&model, current_word));
    });
    siv.call_on_id(consts::ENTRY, |view: &mut EditView| {
        let _callback = view.set_content(current_word);
    });
}

pub fn update_display_on_start(siv: &mut Cursive) {
    siv.call_on_id(consts::CORE, |view: &mut HideableView<LinearLayout>| {
        view.unhide();
    });
    siv.call_on_id(consts::STACK, |view: &mut StackView| {
        view.pop_layer();
    });
    siv.focus_id(consts::ENTRY).unwrap();
}

pub fn update_performance_display(siv: &mut Cursive, performance_monitor: &PerformanceMonitor) {
    siv.call_on_id(consts::PERFORMANCE, |view: &mut TextView| {
        view.set_content(performance_monitor.to_string());
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
    current_entry: &str,
) -> Vec<StyledString> {
    let get_entry = |pos: usize| match pos.cmp(&history.len()) {
        Ordering::Less => &history[pos],
        Ordering::Equal => current_entry,
        Ordering::Greater => &words[pos],
    };
    let get_prefix_style = |pos: usize| match pos.cmp(&history.len()) {
        Ordering::Less => {
            if history[pos] != words[pos] && history[pos].starts_with(&words[pos]) {
                *consts::INCORRECT_STYLE
            } else {
                *consts::CORRECT_STYLE
            }
        }
        Ordering::Equal => *consts::CORRECT_STYLE,
        Ordering::Greater => *consts::FUTURE_STYLE,
    };
    let get_suffix_style = |pos: usize| match pos.cmp(&history.len()) {
        Ordering::Less => *consts::INCORRECT_STYLE,
        Ordering::Equal => *consts::CURRENT_STYLE,
        Ordering::Greater => *consts::FUTURE_STYLE,
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
    current_entry: &str,
    row_width: usize,
) -> StyledString {
    let styled_words = get_styled_words(words, history, current_entry);
    styled_words
        .chunks(row_width)
        .map(|chunk| {
            chunk.iter().with_position().fold(
                SpannedString::new(),
                |mut acc, positioned_styled_word| {
                    match positioned_styled_word {
                        Position::First(word) | Position::Only(word) => {
                            acc.append(word.clone());
                        }
                        Position::Middle(word) | Position::Last(word) => {
                            acc.append(" ");
                            acc.append(word.clone());
                        }
                    }
                    acc
                },
            )
        })
        .with_position()
        .fold(SpannedString::new(), |mut acc, positioned_row| {
            match positioned_row {
                Position::First(row) | Position::Only(row) => {
                    acc.append(row);
                }
                Position::Middle(row) | Position::Last(row) => {
                    acc.append("\n");
                    acc.append(row);
                }
            }
            acc
        })
}

fn get_styled_display<I>(model: &Model<I>, current_word: &str) -> StyledString
where
    I: Iterator<Item = String>,
{
    let words = model.get_words();
    let history = model.get_history();
    get_styled_string(&words, &history, current_word, consts::PANEL_COLS)
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
        let styled_string: StyledString = get_styled_string(&words, &history, "qu", 3);
        let mut expected = SpannedString::new();
        expected.append(SpannedString::styled("sphinx", *consts::CORRECT_STYLE));
        expected.append(SpannedString::styled("", *consts::INCORRECT_STYLE));
        expected.append(" ");
        expected.append(SpannedString::styled("of", *consts::INCORRECT_STYLE));
        expected.append(SpannedString::styled("", *consts::INCORRECT_STYLE));
        expected.append(" ");
        expected.append(SpannedString::styled("bl", *consts::CORRECT_STYLE));
        expected.append(SpannedString::styled("ack", *consts::INCORRECT_STYLE));
        expected.append("\n");
        expected.append(SpannedString::styled("qu", *consts::CORRECT_STYLE));
        expected.append(SpannedString::styled("artz", *consts::CURRENT_STYLE));
        expected.append(" ");
        expected.append(SpannedString::styled("judge", *consts::FUTURE_STYLE));
        expected.append(SpannedString::styled("", *consts::FUTURE_STYLE));
        assert_eq!(styled_string, expected);
    }
}
