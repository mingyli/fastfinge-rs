#[macro_use]
extern crate lazy_static;

use cursive::traits::*;
use cursive::views::{Dialog, DummyView, EditView, LinearLayout, TextView};
use cursive::Cursive;
use rand;
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter;

mod fastfingers;
use fastfingers::model::Model;
use fastfingers::{controller, view};

pub const PANEL_WIDTH: usize = 65;
pub const PANEL_ROWS: usize = 2;

fn get_lexicon<R: BufRead>(reader: &mut R) -> Vec<String> {
    reader.lines().filter_map(Result::ok).collect()
}

fn main() -> io::Result<()> {
    let file = File::open("./input/top1000.txt")?;
    let mut reader = BufReader::new(file);

    let lexicon = get_lexicon(&mut reader);
    let mut rng = rand::thread_rng();
    let it =
        iter::repeat_with(move || lexicon.choose(&mut rng).unwrap().to_owned());
    let mut model = Model::new(it);

    let mut siv: Cursive = Cursive::default();
    let display = TextView::new(view::get_styled_display(&model, ""))
        .with_id("display")
        .fixed_size((PANEL_WIDTH, PANEL_ROWS));
    let entry = EditView::new()
        .on_edit_mut(move |siv: &mut cursive::Cursive, contents, _cursor| {
            controller::on_edit(&mut model, siv, contents, _cursor);
        })
        .with_id("entry");
    let performance =
        TextView::new("").with_id("performance").fixed_size((0, 1));
    siv.add_layer(
        LinearLayout::horizontal()
            .child(
                Dialog::around(
                    LinearLayout::vertical()
                        .child(display)
                        .child(DummyView)
                        .child(entry),
                )
                .title("fastfinge-rs"),
            )
            .child(DummyView)
            .child(DummyView)
            .child(
                Dialog::around(performance)
                    .title("performance")
                    .fixed_width(30),
            ),
    );

    siv.run();
    Ok(())
}
