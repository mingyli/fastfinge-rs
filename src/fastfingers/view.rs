use itertools::{Itertools, Position};
use std::cmp::Ordering;
use std::iter::FusedIterator;

use cursive::theme::Style;
use cursive::traits::{Boxable, Identifiable};
use cursive::utils::markup::StyledString;
use cursive::utils::span::SpannedString;
use cursive::views::{
    Dialog, DummyView, EditView, HideableView, IdView, LinearLayout, StackView, TextView,
};
use cursive::Cursive;

use super::consts;
use super::Model;
use super::PerformanceMonitor;

pub fn update<I>(
    model: &Model<I>,
    performance_monitor: &PerformanceMonitor,
    siv: &mut Cursive,
    current_word: &str,
) where
    I: FusedIterator<Item = String>,
{
    siv.call_on_id(consts::DISPLAY, |view: &mut TextView| {
        view.set_content(get_styled_display(&model, current_word));
    });
    siv.call_on_id(consts::ENTRY, |view: &mut EditView| {
        let _callback = view.set_content(current_word);
    });
    update_performance(siv, performance_monitor);
}

pub fn update_performance(siv: &mut Cursive, performance_monitor: &PerformanceMonitor) {
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

pub fn get_styled_display<I>(model: &Model<I>, current_word: &str) -> StyledString
where
    I: FusedIterator<Item = String>,
{
    let words = model.get_words();
    let history = model.get_history();
    get_styled_string(&words, &history, current_word, consts::PANEL_WIDTH)
}

pub struct ViewBuilder<F, G>
where
    F: FnMut(&mut Cursive, &str, usize) + 'static,
    G: Fn(&mut Cursive) + 'static,
{
    initial_words: Vec<String>,
    edit_callback: Option<Box<F>>,
    start_callback: Option<Box<G>>,
}

impl<F, G> ViewBuilder<F, G>
where
    F: FnMut(&mut Cursive, &str, usize) + 'static,
    G: Fn(&mut Cursive) + 'static,
{
    pub fn new() -> ViewBuilder<F, G> {
        ViewBuilder {
            initial_words: Vec::new(),
            edit_callback: None,
            start_callback: None,
        }
    }

    pub fn with_initial_words(mut self, words: &[String]) -> ViewBuilder<F, G> {
        self.initial_words = words.to_vec();
        self
    }

    pub fn with_edit_callback(mut self, edit_callback: F) -> ViewBuilder<F, G> {
        self.edit_callback = Some(Box::new(edit_callback));
        self
    }

    pub fn with_start_callback(mut self, start_callback: G) -> ViewBuilder<F, G> {
        self.start_callback = Some(Box::new(start_callback));
        self
    }

    pub fn build(self) -> IdView<StackView> {
        let display = TextView::new(get_styled_string(
            &self.initial_words,
            &[],
            "",
            consts::PANEL_WIDTH,
        ))
        .with_id(consts::DISPLAY)
        .fixed_size((consts::PANEL_WIDTH, consts::PANEL_ROWS));

        let performance = TextView::empty()
            .with_id(consts::PERFORMANCE)
            .fixed_size((0, consts::PERFORMANCE_HEIGHT));
        let entry = EditView::new()
            .on_edit_mut(self.edit_callback.unwrap())
            .with_id(consts::ENTRY);
        StackView::new()
            .fullscreen_layer(
                HideableView::new(
                    LinearLayout::horizontal()
                        .child(Dialog::around(
                            LinearLayout::vertical()
                                .child(display)
                                .child(DummyView)
                                .child(entry),
                        ))
                        .child(DummyView)
                        .child(
                            Dialog::around(performance)
                                .title(consts::PERFORMANCE)
                                .fixed_width(consts::PERFORMANCE_WIDTH),
                        ),
                )
                .hidden()
                .with_id(consts::CORE),
            )
            .fullscreen_layer(
                Dialog::new()
                    .title(consts::FAST_FINGERS)
                    .content(TextView::new("Type quickly."))
                    .button("Start", self.start_callback.unwrap()),
            )
            .with_id(consts::STACK)
    }
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
