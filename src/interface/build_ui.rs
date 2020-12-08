use gtk::prelude::*;

use gio::*;
use glib::*;
use gtk::*;

use crate::queueing_system::{analytics, statistics, types};

use super::detail::*;
use super::plotting::*;

pub fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("../glade/main.glade");
    let builder = Builder::from_string(glade_src);

    let window: Window = builder.get_object("mainwindow").expect("Couldn't get mainwindow");
    window.set_application(Some(application));

    window.show_all();
    let sim_button: Button = builder.get_object("go").expect("Couldn't get go button");

    sim_button.connect_clicked(clone!(@weak window => move |_| {
        let numsrc: Entry = builder.get_object("numsrc").expect("Couldn't get numsrc");
        let numdvc: Entry = builder.get_object("numdvc").expect("Couldn't get numdvc");
        let bufcap: Entry = builder.get_object("bufcap").expect("Couldn't get bufcap");
        let avgsrc: Entry = builder.get_object("avgsrc").expect("Couldn't get avgsrc");
        let avgdvc: Entry = builder.get_object("avgdvc").expect("Couldn't get avgdvc");
        let confidence: ComboBox = builder.get_object("confidence").expect("Couldn't get avgdvc");
        println!("{}", confidence.get_active_id().expect("NO"));

        let inp = types::UserInput {
            n_src: numsrc.get_text().as_str().parse::<usize>().unwrap(),
            n_dvc: numdvc.get_text().as_str().parse::<usize>().unwrap(),
            n_buf: bufcap.get_text().as_str().parse::<usize>().unwrap(),
            avg_src: avgsrc.get_text().as_str().parse::<u64>().unwrap(),
            avg_dvc: avgdvc.get_text().as_str().parse::<u64>().unwrap(),
        };

        let confidence_level = get_confidence(confidence.get_active_id().unwrap().as_str()).unwrap();

        let (mut simulations, _) = analytics::get_res(confidence_level, 100, inp, None);
        if simulations.len() == 21 {
            simulations.remove(0);
        }

        let mut x_marks: Vec<f64> = vec![0.0; 21];
        x_marks = x_marks.iter()
                         .enumerate()
                         .map(|(i, _)| i as f64)
                         .collect();

        let mut deny_probs: Vec<f64> = simulations.iter()
                                                .map(|x| statistics::deny_probability(x))
                                                .collect();
        deny_probs.insert(0, 0.);

        construct_and_save(&x_marks, &deny_probs, "Deny prob", "% of reqs processed", "Deny prob");

        let image: Image = builder.get_object("auto_denygraph").expect("No");
        image.set_from_file("target/plot/1.png");
    }));
}
