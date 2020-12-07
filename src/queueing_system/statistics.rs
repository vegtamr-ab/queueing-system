use super::types::*;

pub fn deny_probability(s: &Simulation) -> f64 {
    s.state.requests_denied as f64 / (s.state.requests_processed + s.state.requests_denied) as f64
}

pub fn average_request_time_in_system(s: &Simulation) -> f64 {
    s.state.total_time_spent_in_system as f64 / (s.state.requests_processed + s.state.requests_denied) as f64
}

pub fn usage_coefficient(s: &Simulation) -> f64 {
    s.state.total_time_devices_busy as f64 / (s.max_devices as u64 * s.current_time) as f64
}

pub fn src_deny_probability(s: &Simulation, src_num: usize) -> f64 {
    s.state.s_requests_denied[src_num] as f64 / (s.state.s_requests_denied[src_num] + s.state.s_requests_processed[src_num]) as f64
}

pub fn src_avg_request_time_in_system(s: &Simulation, src_num: usize) -> f64 {
    s.state.s_time_spent_in_system[src_num] as f64 / (s.state.s_requests_denied[src_num] + s.state.s_requests_processed[src_num]) as f64
}

pub fn src_avg_request_time_in_buffer(s: &Simulation, src_num: usize) -> f64 {
    s.state.s_time_spent_in_buffer[src_num] as f64 / (s.state.s_requests_denied[src_num] + s.state.s_requests_processed[src_num]) as f64
}

pub fn src_avg_devices_busy(s: &Simulation, src_num: usize) -> f64 {
    s.state.s_time_devices_busy[src_num] as f64 / (s.state.s_requests_denied[src_num] + s.state.s_requests_processed[src_num]) as f64
}
