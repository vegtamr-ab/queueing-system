mod qs_types;
use qs_types::*;

fn simulator(s: Simulation) -> Simulation {
    match s.next_event {
        SimulationEvent::StopSimulation => s,
        SimulationEvent::RequestArrival => Simulation {
            state: State {

            },
            current_time: ,
            next_event: ,
        },
        SimulationEvent::ProcessRequest => Simulation {
            state: State {

            },
            current_time: ,
            next_event: ,
        }
    }
}

fn get_next_event_and_time(s: State) -> (SimulationEvent, u64) {
    if s.requests_left == 0 {
        (SimulationEvent::StopSimulation, s.next_idle_at)
    } else {
        
    }
}