#[macro_use]
extern crate lazy_static;

use cursive::traits::*;
use cursive::views::{Dialog, DummyView, EditView, LinearLayout, TextView};
use cursive::Cursive;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

mod fastfingers;
use fastfingers::{controller, model, view};

fn get_lexicon<R: BufRead>(reader: &mut R) -> io::Result<Vec<String>> {
    Ok(reader.lines().filter_map(Result::ok).collect())
}

fn main() -> io::Result<()> {
    let file = File::open("./input/top1000.txt")?;
    let mut reader = BufReader::new(file);
    let lexicon = get_lexicon(&mut reader)?;
    let mut model: model::Model =
        model::Model::new().with_lexicon(&lexicon).with_width(8);

    let mut siv: Cursive = Cursive::default();
    let display = TextView::new(view::get_styled_display(&model, ""))
        .with_id("display")
        .fixed_size((0, 2));
    let entry = EditView::new()
        .on_edit_mut(move |siv: &mut cursive::Cursive, contents, _cursor| {
            controller::on_edit(&mut model, siv, contents, _cursor);
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
        .fixed_width(60),
    );

    siv.run();
    Ok(())
}
