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
    RequestArrival,
    ProcessRequest,
    StopSimulation,
}

#[derive(Copy, Clone, Eq)]
pub struct Request {
    pub processing_time_in_ms: u64,
}

#[derive(Copy, Clone)]
pub struct Source {
    pub cooldown: f64,
    pub next_arrival_at: u64,
    pub ready: u8,
}

#[derive(Copy, Clone)]
pub struct Device {
    pub next_idle_at: u64,
    pub ready: u8,
}

#[derive(Eq)]
pub struct State {
    pub sources: Vec<Source>,
    pub max_sources: u32,
    pub devices: Vec<Device>,
    pub device_pointer: u32,
    pub max_devices: u32,
    pub buf: Vec<Request>,
    pub buf_pointer: u32,
    pub buf_max_length: u32,
    pub next_idle_at: u64,
    pub requests_left: u32,
}

#[derive(Eq)]
pub struct Simulation {
    pub state: State,
    pub current_time: u64,
    pub next_event: SimulationEvent,
}

pub struct Res {
    pub deny_probability: f64,
    pub average_time: f64,
    pub usage_coefficient: f64,
}