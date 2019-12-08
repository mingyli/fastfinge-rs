use cursive::traits::*;
use cursive::views::{Dialog, DummyView, EditView, LinearLayout, TextView};
use cursive::Cursive;

mod fastfingers;

fn main() {
    let lexicon: Vec<String> = vec![
        "the", "quick", "brown", "fox", "jumps", "over", "the", "lazy", "dog",
    ]
    .iter()
    .cloned()
    .map(String::from)
    .collect();
    let mut model: fastfingers::model::Model = fastfingers::model::Model::new()
        .with_lexicon(&lexicon)
        .with_width(10);

    let mut siv: Cursive = Cursive::default();
    let display = TextView::new(fastfingers::view::render_display(&model, &""))
        .with_id("display")
        .fixed_size((0, 2));
    let entry = EditView::new()
        .on_edit_mut(move |siv: &mut cursive::Cursive, contents, _cursor| {
            fastfingers::controller::on_edit(
                &mut model, siv, contents, _cursor,
            );
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
