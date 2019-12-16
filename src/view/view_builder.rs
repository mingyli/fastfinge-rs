use cursive::traits::{Boxable, Identifiable};
use cursive::views::{
    Dialog, DummyView, EditView, HideableView, IdView, LinearLayout, StackView, TextView,
};
use cursive::Cursive;

use crate::consts;

#[derive(Default)]
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
        let display = TextView::empty()
            .with_id(consts::DISPLAY)
            .fixed_size((consts::PANEL_COLS, consts::PANEL_ROWS));

        let performance = TextView::empty()
            .with_id(consts::PERFORMANCE)
            .fixed_size((0, consts::PERFORMANCE_ROWS));
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
                                .fixed_width(consts::PERFORMANCE_COLS),
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
