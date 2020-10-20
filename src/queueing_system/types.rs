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
pub struct UserInput {
    pub n_src: usize,
    pub n_dvc: usize,
    pub n_buf: usize,
    pub avg_src: u64,
    pub avg_dvc: u64,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum SimulationEvent {
    PutNewRequestToFullBuffer,
    PutNewRequestToBuffer,
    ProcessNewRequest,
    ProcessRequestFromBuffer,
    StopSimulation,
}

#[derive(Clone, Debug)]
pub struct State {
    pub sources: Vec<u64>,                 //time of next arrival
    pub devices: Vec<u64>,                 //time of next idle state
    pub device_pointer: usize,
    pub buf: Vec<Option<u64>>,             //time of arrival
    pub buf_pointer: usize,
    pub next_idle_at: u64,
    pub next_any_idle_at: u64,
    pub next_arrival_at: u64,
    pub requests_processed: usize,
    pub requests_left: usize,
    pub requests_denied: usize,
    pub total_time_in_buffer: u64,
    pub total_time_devices_busy: u64,
    pub total_time_spent_in_system: u64,
}

#[derive(Clone, Debug)]
pub struct Simulation {
    /* BEGIN OF USER INPUT */
    pub max_sources: usize,
    pub max_devices: usize,
    pub max_buf_length: usize,
    pub average_arrival_cd: u64,
    pub average_device_cd: u64,
    /* END   OF USER INPUT */
    /* BEGIN OF SIM STATE  */
    pub state: State,
    pub current_event: SimulationEvent,
    pub current_time: u64,
    /* END   OF SIM STATE  */
}