#![feature(once_cell)]
mod interface;
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

use interface::{build_ui};
use queueing_system::{analytics, statistics, types};

use std::env::args;

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.builder_basics"),
        Default::default(),
    )
    .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui::build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());

//    let inp = types::UserInput {
//         n_src: 20,
//         n_dvc: 15,
//         n_buf: 5,
//         avg_src: 320,
//         avg_dvc: 560,
//     };

//     let sims = analytics::get_res(types::ConfidenceLevel::Standard, 100, inp, None).0;
//     for final_sim in sims {
//         println!("{}: deny prob", statistics::deny_probability(&final_sim));
//         println!("{}: avg time in sys", statistics::average_request_time_in_system(&final_sim));
//         println!("{}: device coeff", statistics::usage_coefficient(&final_sim));
//     }

    // for i in 0..inp.n_src {
    //     println!("{}: deny prob for src no. {}", statistics::src_deny_probability(&final_sim, i), i);
    //     println!("{}: avg time in sys for src no. {}", statistics::src_avg_request_time_in_system(&final_sim, i), i);
    //     println!("{}: avg time in buf for src no. {}", statistics::src_avg_request_time_in_buffer(&final_sim, i), i);
    //     println!("{}: avg time waiting for src no. {}", statistics::src_avg_devices_busy(&final_sim, i), i);
    // }
}
