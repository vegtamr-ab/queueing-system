mod types;
use types::*;

use lazy::*;

fn simulator(s: Simulation) -> Simulation {
    let state = lazy!(get_new_state(s));

    match s.current_event {
        SimulationEvent::StopSimulation => s,
        SimulationEvent::NewRequest => Simulation {
            state: *state,
            current_event: get_next_event_and_time(*state).0,
            current_time: get_next_event_and_time(*state).1,
        },
        SimulationEvent::ProcessRequest => Simulation {
            state: *state,
            current_event: get_next_event_and_time(*state).0,
            current_time: get_next_event_and_time(*state).1,
        },
    }
}

fn get_new_state(s: Simulation) -> State {
    match s.current_event {
        SimulationEvent::StopSimulation => s.state,
        SimulationEvent::NewRequest => State {
            sources: s.state.sources/************ */,
            max_sources: s.state.max_sources,
            devices: s.state.devices,
            device_pointer: s.state.device_pointer,
            max_devices: s.state.max_devices,
            buf: Vec::new(),
            buf_pointer: 0,
            buf_max_length: s.state.buf_max_length,
            next_idle_at: s.state.devices.iter().max().expect("No devices"),
            next_any_idle_at: s.state.devices.iter().min().expect("No devices"),
            next_arrival_at: s.state.sources.iter().min().expect("No sources"),
            requests_left: s.state.requests_left,
            requests_denied: 0,
            total_time_spent_in_system: 0,
            total_times_devices_busy: s.state.total_time_devices_busy,
        },
        SimulationEvent::ProcessRequest => State {
            sources: s.state.sources,
            max_sources: s.state.max_sources,
            devices: Vec::new(),
            device_pointer: 0,
            max_devices: s.state.max_devices,
            buf: Vec::new(),
            buf_pointer: 0,
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

fn get_new_buffer(st: State) -> (Vec<Option<u64>>, usize) {
    let max = lazy!(st.buf.iter().max.unwrap());

    match st.next_arrival_at.cmp(&st.next_any_idle_at) {
        Less if st.buf.iter().all(|x| x.is_some()) => (),
        Less                                       => (),
        _ if st.buf.iter().all(|x| x.is_none())    => (st.buf, st.buf_pointer),
        _                                          => (st.buf.iter()
                                                             .map(|x| if x == *max { &None } else { x })
                                                             .cloned()
                                                             .collect(),
                                                       st.buf_pointer),
    } 
}

fn add_to_buffer(st: State) -> Vec<Option<u64>> {
    let pos = lazy!(&st.buf[st.buf_pointer..].iter().position(|x| x.is_none()));
    let pos_initial = lazy!(st.buf.iter().position(|x| x.is_none()));

    match *pos {
        Some(a) => st.buf.iter()
                         .enumerate()
                         .map(|(i, x)| if i == (a + st.buf_pointer) { &Some(st.next_arrival_at) } else { x })
                         .cloned()
                         .collect(),
        None => ,
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