/* confidence level:
    Low - 90%
    Medium - 95%
    High - 99%
    Very High - 99.9% */
#[derive(Copy, Clone)]
pub enum ConfidenceLevel {
    Low = 100,
    Medium = 50,
    High = 10,
    VeryHigh = 1,
}

#[derive(Copy, Clone, Eq)]
pub enum SimulationEvent {
    NewRequest,
    ProcessRequest,
    ExtendBuffer,
    StopSimulation,
}

#[derive(Copy, Clone, Eq)]
pub struct Request {
    pub processing_time_in_ms: u64,
}

#[derive(Eq)]
pub struct State {
    pub sources: Vec<u64>,
    pub max_sources: u32,
    pub devices: Vec<u64>,
    pub device_pointer: u32,
    pub max_devices: u32,
    pub buf: Vec<Request>,
    pub buf_pointer: u32,
    pub buf_max_length: u32,
    pub next_idle_at: u64,
    pub next_any_idle_at: u64,
    pub next_arrival_at: u64,
    pub requests_left: u32,
    pub requests_denied: u32,
    pub total_time_spent_in_system: u64,
    pub total_time_devices_busy: u64,
}

#[derive(Eq)]
pub struct Simulation {
    pub state: State,
    pub current_event: SimulationEvent,
    pub current_time: u64,
}

pub struct Res {
    pub deny_probability: f64,
    pub average_time: f64,
    pub usage_coefficient: f64,
}