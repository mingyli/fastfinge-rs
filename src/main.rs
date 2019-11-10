mod model;

use cursive::traits::*;
use cursive::views::{Dialog, DummyView, EditView, LinearLayout, TextView};
use cursive::Cursive;

fn main() {
    let lexicon: Vec<&str> = vec![
        "the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog",
    ];
    let mut model: model::Model =
        model::Model::new().with_lexicon(lexicon).with_size(15);

    let mut siv: Cursive = Cursive::default();
    let display = TextView::new(model.styled_string(""))
        .with_id("display")
        .fixed_size((0, 2));
    let entry = EditView::new()
        .on_edit_mut(move |s: &mut cursive::Cursive, contents, _cursor| {
            model.on_edit(s, contents, _cursor);
        })
        .with_id("entry");
    let performance =
        TextView::new("").with_id("performance").fixed_size((0, 1));
    siv.add_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(display)
                .child(DummyView)
                .child(entry)
                .child(performance),
        )
        .title("fastfinge-rs")
        .fixed_width(50),
    );

    siv.run();
}
