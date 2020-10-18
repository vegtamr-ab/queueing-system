#![feature(once_cell)]
mod queueing_system;

use queueing_system::simulation::*;
use queueing_system::types::*;

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
    let n_src: usize = 5;
    let n_dvc: usize = 10;
    let n_buf: usize = 3;
    let avg_src: u64 = 320;
    let avg_dvc: u64 = 560;
    let st = State {
        sources: vec![0; n_src],
        max_sources: n_src,
        average_arrival_cd: avg_src,
        devices: vec![0; n_dvc],
        device_pointer: 0,
        max_devices: n_dvc,
        average_device_cd: avg_dvc,
        buf: vec![None; n_buf],
        buf_pointer: 0,
        buf_max_length: n_buf,
        next_idle_at: 0,
        next_any_idle_at: 0,
        next_arrival_at: 0,
        requests_processed: 0,
        requests_left: 100,
        requests_denied: 0,
        total_time_in_buffer: 0,
        total_time_devices_busy: 0,
        total_time_spent_in_system: 0,
    };
    let mut s = Simulation {
        state: st,
        current_event: SimulationEvent::NewRequest,
        current_time: 0,
    };
    while s.current_event != SimulationEvent::StopSimulation {
        s = simulator(&s);
    }
    println!("{:?}", s);
}