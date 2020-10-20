#![feature(once_cell)]
mod queueing_system;

use queueing_system::{analytics, statistics, types};

use druid::{AppLauncher, WindowDesc, Widget, PlatformError};
use druid::widget::{Label, Flex, Padding, Align};

fn build_ui() -> impl Widget<()> {
    Padding::new(
        10.0,
        Flex::row()
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new("top left"), 1.0)
                    .with_flex_child(Align::centered(Label::new("bottom left")), 1.0),
                1.0)
            .with_flex_child(
                Flex::column()
                    .with_flex_child(Label::new("top right"), 1.0)
                    .with_flex_child(Align::centered(Label::new("bottom right")), 1.0),
                1.0))
}

fn main() {
    let inp = types::UserInput {
        n_src: 5,
        n_dvc: 8,
        n_buf: 3,
        avg_src: 320,
        avg_dvc: 560,
    };

    let (final_sim, final_n) = analytics::get_res(types::ConfidenceLevel::Low, 100, inp, None);
    //println!("{:?}", final_sim);
    println!("{:?}", final_n);
    println!("{}: deny prob", statistics::deny_probability(&final_sim));
    println!("{}: avg time in sys", statistics::average_request_time_in_system(&final_sim));
    println!("{}: device coeff", statistics::usage_coefficient(&final_sim));
}
