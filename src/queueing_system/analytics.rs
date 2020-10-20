use super::simulation::*;
use super::types::*;

fn get_student_value(cl: ConfidenceLevel) -> f64 {
    match cl {
        ConfidenceLevel::Low => 1.6649,
        ConfidenceLevel::Medium => 1.96,
        ConfidenceLevel::High => 2.5758,
        ConfidenceLevel::VeryHigh => 3.2905,
    }
}

fn number_of_entries(cl: ConfidenceLevel, p: f64) -> usize {
    let delta = (cl as i32) as f64 / 1000.0;
    let st_value = get_student_value(cl);

    ((st_value * st_value * (1.0 - p)) / (p * delta * delta)) as usize
}

fn base_simulation(n: usize, inp: UserInput) -> Simulation {
    Simulation {
        max_sources: inp.n_src,
        max_devices: inp.n_dvc,
        max_buf_length: inp.n_buf,
        average_arrival_cd: inp.avg_src,
        average_device_cd: inp.avg_dvc,
        state: State {
            sources: vec![0; inp.n_src],
            devices: vec![0; inp.n_dvc],
            device_pointer: 0,
            buf: vec![None; inp.n_buf],
            buf_pointer: 0,
            next_idle_at: 0,
            next_any_idle_at: 0,
            next_arrival_at: 0,
            requests_processed: 0,
            requests_left: n,
            requests_denied: 0,
            total_time_in_buffer: 0,
            total_time_devices_busy: 0,
            total_time_spent_in_system: 0,
        },
        current_event: SimulationEvent::ProcessNewRequest,
        current_time: 0,
    }
}

fn simulation_cycle(s: &Simulation) -> Simulation {
    let mut sim = s.clone();
    while sim.current_event != SimulationEvent::StopSimulation {
        sim = simulator(&sim);
    }
    sim
}

fn deny_probability(s: &Simulation) -> f64 {
    s.state.requests_denied as f64 / (s.state.requests_processed + s.state.requests_denied) as f64
}

pub fn get_res(cl: ConfidenceLevel, n: usize, inp: UserInput, sim: Option<Simulation>) -> (Simulation, usize) {
    let s = match sim {
        Some(a) => a,
        None => simulation_cycle(&base_simulation(n, inp)), 
    };
    let p = deny_probability(&s);
    let new_n = if p != 0.0 {
        number_of_entries(cl, p)
    } else {
        n * 10
    };
    let new_s = simulation_cycle(&base_simulation(new_n, inp));
    let new_p = deny_probability(&new_s);

    if (new_p - p).abs() > (p * 0.1) {
        get_res(cl, new_n, inp, Some(new_s))
    } else {
        (s, n)
    }
}