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
