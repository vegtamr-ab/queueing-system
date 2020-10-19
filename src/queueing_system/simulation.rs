use itertools::Itertools;
use std::cmp::Ordering::*;
use std::lazy::Lazy;

use super::types::*;

pub fn simulator(s: &Simulation) -> Simulation {
    let state = Lazy::new(|| get_new_state(s));
    let get_next_et = Lazy::new(|| get_next_event_and_time(&*state));

    match s.current_event {
        SimulationEvent::StopSimulation => s.clone(),
        _ => Simulation {
            max_sources: s.max_sources,
            max_devices: s.max_devices,
            max_buf_length: s.max_buf_length,
            average_arrival_cd: s.average_arrival_cd,
            average_device_cd: s.average_device_cd,
            state: state.clone(),
            current_event: get_next_et.0,
            current_time: get_next_et.1,
        },
    }
}

fn get_new_state(s: &Simulation) -> State {
    let arrival_time = Lazy::new(|| get_new_arrival_time(&s));
    let idle_time = Lazy::new(|| get_new_idle_time(&s));
    let new_sources = Lazy::new(|| update_sources(&s, *arrival_time));
    let new_devices = Lazy::new(|| update_devices(&s, *idle_time));
    let new_buffer = Lazy::new(|| update_buffer(&s));
    let new_requests = Lazy::new(|| update_requests(&s));
    let new_denied = Lazy::new(|| update_denied(&s));
    let new_buffer_time = Lazy::new(|| update_buffer_time(s));
    let new_busy = Lazy::new(|| update_busy_time(&s, *idle_time));

    match s.current_event {
        SimulationEvent::StopSimulation => s.state.clone(),
        _ => State {
            sources: new_sources.clone(),
            devices: new_devices.0.clone(),
            device_pointer: new_devices.1 % s.max_devices,
            buf: new_buffer.0.clone(),
            buf_pointer: new_buffer.1 % s.max_buf_length,
            next_idle_at: *new_devices.0.iter().max().expect("No devices"),
            next_any_idle_at: *new_devices.0.iter().min().expect("No devices"),
            next_arrival_at: *new_sources.iter().min().expect("No sources"),
            requests_processed: new_requests.0,
            requests_left: new_requests.1,
            requests_denied: *new_denied,
            total_time_in_buffer: *new_buffer_time,
            total_time_devices_busy: *new_busy,
            total_time_spent_in_system: *new_buffer_time + *new_busy,
        },
    }
}

fn update_sources(s: &Simulation, new_arrival_time: u64) -> Vec<u64> {
    let updated = Lazy::new(|| new_sources(s, new_arrival_time));

    if let SimulationEvent::ProcessRequestFromBuffer = s.current_event {
        s.state.sources.clone()
    } else {
        updated.clone()
    }
}

fn new_sources(s: &Simulation, new_arrival_time: u64) -> Vec<u64> {
    let min = Lazy::new(|| s.state.sources.iter().min().unwrap());
    let min_pos = Lazy::new(|| s.state.sources.iter().position(|x| x == *min));
    let new_arrival = Lazy::new(|| *min + new_arrival_time);

    s.state.sources.iter()
              .enumerate()
              .map(|(i, x)| if i == min_pos.unwrap() { &new_arrival } else { x })
              .cloned()
              .collect()
}

fn get_new_arrival_time(s: &Simulation) -> u64 {
    if rand::random() {
        s.average_arrival_cd + (rand::random::<f64>() * s.average_arrival_cd as f64 * 0.1).round() as u64
    } else {
        (s.average_arrival_cd - (rand::random::<f64>() * s.average_arrival_cd as f64 * 0.1).round()).abs() as u64
    }
}

fn update_devices(s: &Simulation, new_idle_time: u64) -> (Vec<u64>, usize) {
    let pick = Lazy::new(|| pick_device(s, new_idle_time));

    match s.current_event {
        SimulationEvent::ProcessNewRequest
      | SimulationEvent::ProcessRequestFromBuffer => (pick.0.clone(), pick.1),
        _                                         => (s.state.devices.clone(), s.state.device_pointer),
    }
}

fn pick_device(s: &Simulation, new_idle_time: u64) -> (Vec<u64>, usize) {
    let free_pos = Lazy::new(|| (&s.state.devices[s.state.device_pointer..]).iter().position(|x| *x <= s.current_time));
    let free_pos_initial = Lazy::new(|| s.state.devices.iter().position(|x| *x <= s.current_time));
    let new_idle = Lazy::new(|| s.current_time + new_idle_time);

    match *free_pos {
        Some(a) => (s.state.devices.iter()
                                .enumerate()
                                .map(|(i, x)| if i == (a + s.state.device_pointer) { &new_idle } else { x })
                                .cloned()
                                .collect(),
                    a + s.state.device_pointer + 1),
        None    => (s.state.devices.iter()
                                .enumerate()
                                .map(|(i, x)| if i == free_pos_initial.unwrap() { &new_idle } else { x })
                                .cloned()
                                .collect(),
                    free_pos_initial.unwrap() + 1),
    }
}

fn get_new_idle_time(s: &Simulation) -> u64 {
    (-1f64 * s.average_device_cd as f64 * rand::random::<f64>().ln()).round() as u64
}

fn update_buffer(s: &Simulation) -> (Vec<Option<u64>>, usize) {
    let max = Lazy::new(|| s.state.buf.iter().max().unwrap());
    let min = Lazy::new(|| s.state.buf.iter().filter(|x| x.is_some()).min().unwrap());
    let min_pos = Lazy::new(|| s.state.buf.iter().position(|x| x == *min));
    let add = Lazy::new(|| add_to_buffer(s));
    let some_arrival = Some(s.state.next_arrival_at);

    match s.current_event {
        SimulationEvent::PutNewRequestToFullBuffer => (s.state.buf.iter()
                                                             .map(|x| if x == *min { &some_arrival } else { x })
                                                             .cloned()
                                                             .collect(),
                                                       min_pos.unwrap() + 1),
        SimulationEvent::PutNewRequestToBuffer     => (add.0.clone(), add.1),
        SimulationEvent::ProcessNewRequest         => (s.state.buf.clone(), s.state.buf_pointer),
        SimulationEvent::ProcessRequestFromBuffer  => (s.state.buf.iter()
                                                             .map(|x| if x == *max { &None } else { x })
                                                             .cloned()
                                                             .collect(),
                                                       s.state.buf_pointer),
        SimulationEvent::StopSimulation            => (s.state.buf.clone(), s.state.buf_pointer),
    } 
}

fn add_to_buffer(s: &Simulation) -> (Vec<Option<u64>>, usize) {
    let pos = Lazy::new(|| (&s.state.buf[s.state.buf_pointer..]).iter().position(|x| x.is_none()));
    let pos_initial = Lazy::new(|| s.state.buf.iter().position(|x| x.is_none()));
    let some_arrival = Some(s.state.next_arrival_at);

    match *pos {
        Some(a) => (s.state.buf.iter()
                         .enumerate()
                         .map(|(i, x)| if i == (a + s.state.buf_pointer) { &some_arrival } else { x })
                         .cloned()
                         .collect(),
                    a + s.state.buf_pointer + 1),
        None    => (s.state.buf.iter()
                         .enumerate()
                         .map(|(i, x)| if i == pos_initial.unwrap() { &some_arrival } else { x })
                         .cloned()
                         .collect(),
                    pos_initial.unwrap() + 1),
    }
}

fn update_requests(s: &Simulation) -> (u32, u32) {
    match s.current_event {
        SimulationEvent::PutNewRequestToFullBuffer => (s.state.requests_processed, s.state.requests_left - 1),
        SimulationEvent::PutNewRequestToBuffer     => (s.state.requests_processed, s.state.requests_left), 
        _                                          => (s.state.requests_processed + 1, s.state.requests_left - 1),
    }
}

fn update_denied(s: &Simulation) -> u32 {
    if let SimulationEvent::PutNewRequestToFullBuffer = s.current_event {
        s.state.requests_denied + 1
    } else {
        s.state.requests_denied
    }
}

fn update_buffer_time(s: &Simulation) -> u64 {
    let max = Lazy::new(|| s.state.buf.iter().max().unwrap());
    let min = Lazy::new(|| s.state.buf.iter().filter(|x| x.is_some()).min().unwrap());

    match s.current_event {
        SimulationEvent::PutNewRequestToFullBuffer => s.state.total_time_in_buffer + (s.current_time - min.unwrap()),
        SimulationEvent::ProcessRequestFromBuffer  => s.state.total_time_in_buffer + (s.current_time - max.unwrap()),
        _                                          => s.state.total_time_in_buffer,
    }
}

fn update_busy_time(s: &Simulation, idle_time: u64) -> u64 {
    match s.current_event {
        SimulationEvent::ProcessNewRequest
      | SimulationEvent::ProcessRequestFromBuffer => s.state.total_time_devices_busy + idle_time,
        _                                         => s.state.total_time_devices_busy,
    }
}

fn get_next_event_and_time(st: &State) -> (SimulationEvent, u64) {
    if st.requests_left == 0 {
        (SimulationEvent::StopSimulation, st.next_idle_at)
    } else {
        match st.next_arrival_at.cmp(&st.next_any_idle_at) {
            Less if st.buf.iter().all(|x| x.is_some()) => (SimulationEvent::PutNewRequestToFullBuffer, st.next_arrival_at),
            Less                                                     => (SimulationEvent::PutNewRequestToBuffer, st.next_arrival_at),
            _ if st.buf.iter().all(|x| x.is_none())    => (SimulationEvent::ProcessNewRequest, st.next_arrival_at),
            _                                                        => (SimulationEvent::ProcessRequestFromBuffer, st.next_any_idle_at),
        }
    }
}