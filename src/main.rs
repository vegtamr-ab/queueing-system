#![feature(once_cell)]
mod queueing_system;

extern crate gio;
extern crate glib;
extern crate gtk;
extern crate gnuplot;

use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;

use gtk::*;
use gnuplot::*;

use queueing_system::{analytics, statistics, types};

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("glade/main.glade");
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

        let inp = types::UserInput {
            n_src: numsrc.get_text().as_str().parse::<usize>().unwrap(),
            n_dvc: numdvc.get_text().as_str().parse::<usize>().unwrap(),
            n_buf: bufcap.get_text().as_str().parse::<usize>().unwrap(),
            avg_src: avgsrc.get_text().as_str().parse::<u64>().unwrap(),
            avg_dvc: avgdvc.get_text().as_str().parse::<u64>().unwrap(),
        };

        let (final_sim, final_n) = analytics::get_res(types::ConfidenceLevel::Standard, 100, inp, None);

        let textView: TextView = builder.get_object("res").expect("Couldn't get res");
        let contents = "Hellow";
        textView.get_buffer().expect("Couldn't get buf").set_text(&contents);
    }));
}

fn main() {
    // let application = gtk::Application::new(
    //     Some("com.github.gtk-rs.examples.builder_basics"),
    //     Default::default(),
    // )
    // .expect("Initialization failed...");

    // application.connect_activate(|app| {
    //     build_ui(app);
    // });

    // application.run(&args().collect::<Vec<_>>());

   let inp = types::UserInput {
        n_src: 20,
        n_dvc: 15,
        n_buf: 5,
        avg_src: 320,
        avg_dvc: 560,
    };

    let (mut simulations, final_n) = analytics::get_res(types::ConfidenceLevel::Standard, 100, inp, None);
    if simulations.len() == 21 {
        simulations.remove(0);
    }
    println!("{:?}", simulations.len());
    println!("{:?}", final_n);

    let mut deny_probs: Vec<f64> = simulations.iter()
                                              .map(|x| statistics::deny_probability(x))
                                              .collect();
    deny_probs.insert(0, 0.);

    let mut fg = Figure::new();
    fg.axes2d()
        .set_title("Final sim denyprob", &[])
        .set_legend(Graph(0.5), Graph(0.9), &[], &[])
        .set_x_label("% of requests processed", &[])
        .set_y_label("Deny probability", &[])
        .lines(
            &[0., 5., 10., 15., 20., 25., 30., 35., 40., 45., 50., 55., 60., 65., 70., 75., 80., 85., 90., 95., 100.],
            deny_probs.as_slice(),
            &[],
        );
    fg.show().unwrap();

    // println!("{}: deny prob", statistics::deny_probability(&final_sim));
    // println!("{}: avg time in sys", statistics::average_request_time_in_system(&final_sim));
    // println!("{}: device coeff", statistics::usage_coefficient(&final_sim));
    // for i in 0..inp.n_src {
    //     println!("{}: deny prob for src no. {}", statistics::src_deny_probability(&final_sim, i), i);
    //     println!("{}: avg time in sys for src no. {}", statistics::src_avg_request_time_in_system(&final_sim, i), i);
    //     println!("{}: avg time in buf for src no. {}", statistics::src_avg_request_time_in_buffer(&final_sim, i), i);
    //     println!("{}: avg time waiting for src no. {}", statistics::src_avg_devices_busy(&final_sim, i), i);
    // }
}
