#[macro_use]
extern crate lazy_static;

use rand;
use rand::seq::SliceRandom;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

use cursive::views::{HideableView, LinearLayout, StackView};
use cursive::Cursive;

mod fastfingers;
use fastfingers::consts;
use fastfingers::controller;
use fastfingers::view;
use fastfingers::ModelBuilder;
use fastfingers::PerformanceMonitor;
use fastfingers::ViewBuilder;

fn get_lexicon<R: BufRead>(reader: &mut R) -> Vec<String> {
    reader.lines().filter_map(Result::ok).collect()
}

fn main() -> io::Result<()> {
    let file = File::open(consts::INPUT_FILE)?;
    let mut reader = BufReader::new(file);

    let lexicon = get_lexicon(&mut reader);
    let mut rng = rand::thread_rng();
    let word_stream = iter::repeat_with(move || {
        lexicon
            .choose_multiple(&mut rng, 100)
            .cloned()
            .collect::<Vec<String>>()
    })
    .flatten();
    let model = ModelBuilder::new().with_word_stream(word_stream).build();
    let model_arc = Arc::new(RwLock::new(model));

    let performance = PerformanceMonitor::new();
    let performance_arc = Arc::new(RwLock::new(performance));

    let model_on_edit_instance = model_arc.clone();
    let performance_on_edit_instance = performance_arc.clone();
    let performance_on_start_instance = performance_arc.clone();
    let performance_background_instance = performance_arc.clone();

    let view = ViewBuilder::new()
        .with_initial_words(&model_arc.clone().read().unwrap().get_words())
        .with_edit_callback(move |siv: &mut Cursive, contents, _cursor| {
            controller::on_keypress(
                &mut model_on_edit_instance.write().unwrap(),
                &mut performance_on_edit_instance.write().unwrap(),
                siv,
                contents,
                _cursor,
            );
        })
        .with_start_callback(move |siv: &mut Cursive| {
            siv.call_on_id(consts::CORE, |view: &mut HideableView<LinearLayout>| {
                view.unhide();
            });
            siv.call_on_id(consts::STACK, |view: &mut StackView| {
                view.pop_layer();
            });
            performance_on_start_instance
                .write()
                .unwrap()
                .start()
                .expect("The performance monitor should not have been started yet.");
            siv.focus_id(consts::ENTRY).unwrap();
        })
        .build();

    {
        let mut siv = Cursive::default();

        let cb_sink = siv.cb_sink().clone();
        thread::spawn(move || loop {
            let performance_iteration_instance = performance_background_instance.clone();
            cb_sink
                .send(Box::new(move |siv: &mut Cursive| {
                    view::update_performance(siv, &performance_iteration_instance.read().unwrap());
                }))
                .unwrap();
            thread::sleep(std::time::Duration::from_millis(50));
        });

        siv.add_layer(view);
        siv.run();
    }

    let performance_print_summary_instance = performance_arc.clone();
    performance_print_summary_instance
        .write()
        .unwrap()
        .end()
        .expect("The performance monitor should not have been ended yet.");
    let performance = performance_print_summary_instance.read().unwrap();
    println!("{}", performance);

    Ok(())
}
