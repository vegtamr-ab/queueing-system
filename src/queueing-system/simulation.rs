#![feature(once_cell)]

mod types;
use types::*;

use itertools::Itertools;
use std::lazy::Lazy;

fn simulator(s: Simulation) -> Simulation {
    let state = Lazy::new(|| get_new_state(s));
    let get_next_et = Lazy::new(|| get_next_event_and_time(*state));

    match s.current_event {
        SimulationEvent::StopSimulation => s,
        _ => Simulation {
            state: *state,
            current_event: *get_next_et.0,
            current_time: *get_next_et.1,
        },
    }
}

fn get_new_state(s: Simulation) -> State {
    let new_devices = Lazy::new(|| update_devices(st));
    let new_buffer = Lazy::new(|| get_new_buffer(st));

    match s.current_event {
        SimulationEvent::StopSimulation => s.state,
        SimulationEvent::NewRequest => State {
            sources: s.state.sources/************ */,
            max_sources: s.state.max_sources,
            average_arrival_cd: s.state.average_arrival_cd,
            devices: *new_devices.0,
            device_pointer: *new_devices.1,
            max_devices: s.state.max_devices,
            average_device_cd: s.state.average_device_cd,
            buf: *new_buffer.0,
            buf_pointer: *new_buffer.1,
            buf_max_length: s.state.buf_max_length,
            next_idle_at: s.state.devices.iter().max().expect("No devices"),
            next_any_idle_at: s.state.devices.iter().min().expect("No devices"),
            next_arrival_at: s.state.sources.iter().min().expect("No sources"),
            requests_left: s.state.requests_left,
            requests_denied: 0,
            total_time_spent_in_system: 0,
            total_times_devices_busy: 0,
        },
        SimulationEvent::ProcessRequest => State {
            sources: s.state.sources,
            max_sources: s.state.max_sources,
            average_arrival_cd: s.state.average_arrival_cd,
            devices: *new_devices.0,
            device_pointer: *new_devices.1,
            max_devices: s.state.max_devices,
            average_device_cd: s.state.average_device_cd,
            buf: *new_buffer.0,
            buf_pointer: *new_buffer.1,
            buf_max_length: s.state.buf_max_length,
            next_idle_at: s.state.devices.iter().max().expect("No devices"),
            next_any_idle_at: s.state.devices.iter().min().expect("No devices"),
            next_arrival_at: s.state.sources.iter().min().expect("No sources"),
            requests_left: s.state.requests_left - 1,
            requests_denied: 0,
            total_time_spent_in_system: 0,
            total_times_devices_busy: 0,
        },
    }
}

fn update_devices(st: State) -> (Vec<u64>, usize) {
    let pick = Lazy::new(|| pick_device(s));

    match st.next_arrival_at.cmp(&st.next_any_idle_at) {
        Less => (st.devices, st.device_pointer),
        _ => (*pick.0, *pick.1),
    }
}

fn pick_device(st: State) -> (Vec<u64>, usize) {
    let min = Lazy::new(|| st.devices.iter().min().unwrap());
    let min_pos = Lazy::new(|| st.devices.iter().position(|x| x == *min));
    let min_pos_pointer = Lazy::new(|| &st.devices[st.device_pointer..].iter().position(|x| x == *min));
    let new_idle_time = Lazy::new(|| get_new_idle_time(st));

    match st.devices.filter(|x| x == *min).exactly_one() {
        Some(a) => (st.devices.iter()
                              .map(|x| if x == *min { &(*min + *new_idle_time) } else { x })
                              .cloned()
                              .collect(),
                    *min_pos.unwrap() + 1),
        _ => match *min_pos_pointer {
            Some(a) => (st.devices.iter()
                                  .enumerate()
                                  .map(|(i, x)| if i == (a + st.device_pointer) { *min + *new_idle_time } else { x })
                                  .cloned()
                                  .collect(),
                        a + st.device_pointer + 1),
            None    => (st.buf.iter()
                                  .enumerate()
                                  .map(|(i, x)| if i == *min_pos.unwrap() { *min + *new_idle_time } else { x })
                                  .cloned()
                                  .collect(),
                        *min_pos.unwrap() + 1),
        },
    }
}

fn get_new_idle_time(st: State) {
    -1 * st.average_device_cd * (rand::random::<f64>().ln()).round()
}

fn get_new_buffer(st: State) -> (Vec<Option<u64>>, usize) {
    let max = Lazy::new(|| st.buf.iter().max().unwrap());
    let min = Lazy::new(|| st.buf.iter().filter(|x| x.is_some()).min().unwrap());
    let min_pos = Lazy::new(|| st.buf.iter().position(|x| x == *min));
    let add = Lazy::new(|| add_to_buffer(st));

    match st.next_arrival_at.cmp(&st.next_any_idle_at) {
        Less if st.buf.iter().all(|x| x.is_some()) => (st.buf.iter()
                                                             .map(|x| if x == *min { &Some(st.next_arrival_at) } else { x })
                                                             .cloned()
                                                             .collect(),
                                                       *min_pos.unwrap() + 1),
        Less                                       => (*add.0, *add.1),
        _ if st.buf.iter().all(|x| x.is_none())    => (st.buf, st.buf_pointer),
        _                                          => (st.buf.iter()
                                                             .map(|x| if x == *max { &None } else { x })
                                                             .cloned()
                                                             .collect(),
                                                       st.buf_pointer),
    } 
}

fn add_to_buffer(st: State) -> (Vec<Option<u64>>, usize) {
    let pos = Lazy::new(|| &st.buf[st.buf_pointer..].iter().position(|x| x.is_none()));
    let pos_initial = Lazy::new(|| st.buf.iter().position(|x| x.is_none()));

    match *pos {
        Some(a) => (st.buf.iter()
                         .enumerate()
                         .map(|(i, x)| if i == (a + st.buf_pointer) { &Some(st.next_arrival_at) } else { x })
                         .cloned()
                         .collect(),
                    a + st.buf_pointer + 1),
        None    => (st.buf.iter()
                         .enumerate()
                         .map(|(i, x)| if i == *pos_initial.unwrap() { &Some(st.next_arrival_at) } else { x })
                         .cloned()
                         .collect(),
                    *pos_initial.unwrap() + 1),
    }
}

fn get_next_event_and_time(st: State) -> (SimulationEvent, u64) {
    if st.requests_left == 0 {
        (SimulationEvent::StopSimulation, st.next_idle_at)
    } else {
        match st.next_arrival_at.cmp(&st.next_any_idle_at) {
            Less                                    => (SimulationEvent::NewRequest, st.next_arrival_at),
            _ if st.buf.iter().all(|x| x.is_none()) => (SimulationEvent::NewRequest, st.next_arrival_at),
            _                                       => (SimulationEvent::ProcessRequest, st.next_any_idle_at),
        }
    }
}