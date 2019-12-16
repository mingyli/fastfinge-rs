use rand;
use rand::seq::SliceRandom;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::iter;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;

use cursive::Cursive;

use fastfingers::consts;
use fastfingers::controller;
use fastfingers::model::{Model, ModelBuilder};
use fastfingers::performance::PerformanceMonitor;
use fastfingers::view;
use fastfingers::view::ViewBuilder;

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
            .choose_multiple(&mut rng, consts::SAMPLE_SIZE)
            .cloned()
            .collect::<Vec<String>>()
    })
    .flatten();
    let model: Model<_> = ModelBuilder::new().with_word_stream(word_stream).build();
    let model_arc = Arc::new(RwLock::new(model));
    let model_on_edit_instance = model_arc.clone();
    let model_on_start_instance = model_arc.clone();

    let performance = PerformanceMonitor::new();
    let performance_arc = Arc::new(RwLock::new(performance));
    let performance_on_edit_instance = performance_arc.clone();
    let performance_on_start_instance = performance_arc.clone();
    let performance_background_instance = performance_arc.clone();

    let view = ViewBuilder::new()
        .with_initial_words(&model_arc.clone().read().unwrap().get_words())
        .with_edit_callback(move |siv: &mut Cursive, contents, _cursor| {
            controller::on_keypress(
                siv,
                &mut model_on_edit_instance.write().unwrap(),
                &mut performance_on_edit_instance.write().unwrap(),
                contents,
                _cursor,
            );
        })
        .with_start_callback(move |siv: &mut Cursive| {
            controller::on_start(siv, &model_on_start_instance.read().unwrap());
            performance_on_start_instance
                .write()
                .unwrap()
                .start()
                .expect("The performance monitor should not have been started yet.");
        })
        .build();

    {
        let mut siv = Cursive::default();

        let cb_sink = siv.cb_sink().clone();
        thread::spawn(move || loop {
            let performance_iteration_instance = performance_background_instance.clone();
            cb_sink
                .send(Box::new(move |siv: &mut Cursive| {
                    view::update_performance_display(
                        siv,
                        &performance_iteration_instance.read().unwrap(),
                    );
                }))
                .unwrap();
            thread::sleep(std::time::Duration::from_millis(
                consts::PERFORMANCE_REFRESH_MS,
            ));
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
