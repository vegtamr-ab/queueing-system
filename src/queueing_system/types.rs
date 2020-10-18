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

#[derive(Copy, Clone)]
pub enum SimulationEvent {
    NewRequest,
    ProcessRequest,
    StopSimulation,
}

#[derive(Clone)]
pub struct State {
    pub sources: Vec<u64>,                 //time of next arrival
    pub max_sources: usize,
    pub average_arrival_cd: u64,
    pub devices: Vec<u64>,                 //time of next idle state
    pub device_pointer: usize,
    pub max_devices: usize,
    pub average_device_cd: u64,
    pub buf: Vec<Option<u64>>,             //time of arrival
    pub buf_pointer: usize,
    pub buf_max_length: usize,
    pub next_idle_at: u64,
    pub next_any_idle_at: u64,
    pub next_arrival_at: u64,
    pub requests_left: u32,
    pub requests_denied: u32,
    pub total_time_in_buffer: u64,
    pub total_time_devices_busy: u64,
    pub total_time_spent_in_system: u64,
}

#[derive(Clone)]
pub struct Simulation {
    pub state: State,
    pub current_event: SimulationEvent,
    pub current_time: u64,
}