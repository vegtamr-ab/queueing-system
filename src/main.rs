#![feature(once_cell)]
mod queueing_system;

use queueing_system::{analytics, statistics, types};

fn main() {
    let inp = types::UserInput {
        n_src: 20,
        n_dvc: 15,
        n_buf: 5,
        avg_src: 320,
        avg_dvc: 560,
    };

    let (final_sim, final_n) = analytics::get_res(types::ConfidenceLevel::Standard, 100, inp, None);
    //println!("{:?}", final_sim);
    println!("{:?}", final_n);
    println!("{}: deny prob", statistics::deny_probability(&final_sim));
    println!("{}: avg time in sys", statistics::average_request_time_in_system(&final_sim));
    println!("{}: device coeff", statistics::usage_coefficient(&final_sim));
    for i in 0..inp.n_src {
        println!("{}: deny prob for src no. {}", statistics::src_deny_probability(&final_sim, i), i);
        println!("{}: avg time in sys for src no. {}", statistics::src_avg_request_time_in_system(&final_sim, i), i);
        println!("{}: avg time in buf for src no. {}", statistics::src_avg_request_time_in_buffer(&final_sim, i), i);
        println!("{}: avg time waiting for src no. {}", statistics::src_avg_devices_busy(&final_sim, i), i);
    }
}
